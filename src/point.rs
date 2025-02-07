use std::ops::{Add, Mul, Neg};
use thiserror::Error;

use crate::field::FieldElement;

#[derive(Error, Debug)]
pub enum PointError {
    #[error("Point is not on the curve")]
    NotOnCurve,
    #[error("Cannot perform operation with points on different curves")]
    DifferentCurves,
    #[error("Field error: {0}")]
    FieldError(#[from] crate::field::FieldError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: Option<FieldElement>,
    pub y: Option<FieldElement>,
    pub a: FieldElement,
    pub b: FieldElement,
}

/// Pointの生成と操作を定義
impl Point {
    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Result<Self, PointError> {
        // 無限遠点の場合
        if x.is_none() && y.is_none() {
            return Ok(Self { x, y, a, b });
        }

        // 点が曲線上にあるか確認
        if let (Some(x_val), Some(y_val)) = (x.as_ref(), y.as_ref()) {
            let y_squared = *y_val * *y_val;
            let x_cubed = *x_val * *x_val * *x_val;
            let ax = a * *x_val;

            if y_squared == x_cubed + ax + b {
                Ok(Self { x, y, a, b })
            } else {
                Err(PointError::NotOnCurve)
            }
        } else {
            Err(PointError::NotOnCurve)
        }
    }

    pub fn is_infinity(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }
}

/// Pointの加算を定義
impl Add for Point {
    type Output = Result<Self, PointError>;

    fn add(self, other: Self) -> Result<Self, PointError> {
        if self.a != other.a || self.b != other.b {
            return Err(PointError::DifferentCurves);
        }

        // 無限遠点の場合
        if self.is_infinity() {
            return Ok(other);
        }
        if other.is_infinity() {
            return Ok(self);
        }

        let x1 = self.x.unwrap();
        let y1 = self.y.unwrap();
        let x2 = other.x.unwrap();
        let y2 = other.y.unwrap();

        // P + (-P) = O
        if x1 == x2 && y1 == -y2 {
            return Point::new(None, None, self.a, self.b);
        }

        // スロープを計算
        let slope = if x1 == x2 && y1 == y2 {
            // s = (3x₁² + a) / 2y₁
            let numerator = x1 * x1 * FieldElement::new(3, x1.prime())? + self.a;
            let denominator = y1 + y1;
            numerator / denominator
        } else {
            // s = (y₂ - y₁) / (x₂ - x₁)
            (y2 - y1) / (x2 - x1)
        };

        // x₃ = s² - x₁ - x₂
        let x3 = slope * slope - x1 - x2;
        let y3 = slope * (x1 - x3) - y1;

        Point::new(Some(x3), Some(y3), self.a, self.b)
    }
}

/// Pointのスカラー乗算を定義
impl Neg for Point {
    type Output = Self;

    /// Pointの符号を反転
    fn neg(self) -> Self {
        if self.is_infinity() {
            return self;
        }

        Self {
            x: self.x,
            y: self.y.map(|y| -y),
            a: self.a,
            b: self.b,
        }
    }
}

/// Pointのスカラー乗算を定義
impl Mul<i64> for Point {
    type Output = Result<Self, PointError>;

    fn mul(self, scalar: i64) -> Result<Self, PointError> {
        let mut coef = scalar;
        let mut current = self.clone();
        let mut result = Point::new(None, None, self.a, self.b)?;

        while coef > 0 {
            if coef & 1 == 1 {
                result = (result + current.clone())?;
            }
            current = (current.clone() + current)?;
            coef >>= 1;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::FieldElement;

    fn create_test_point() -> Point {
        // Using curve y² = x³ + 7 over F223
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let x = FieldElement::new(192, prime).unwrap();
        let y = FieldElement::new(105, prime).unwrap();
        Point::new(Some(x), Some(y), a, b).unwrap()
    }

    #[test]
    fn test_point_creation() {
        let point = create_test_point();
        assert!(!point.is_infinity());
    }

    #[test]
    fn test_point_addition() {
        // Using curve y² = x³ + 7 over F223
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();

        // Point 1: (192, 105)
        let x1 = FieldElement::new(192, prime).unwrap();
        let y1 = FieldElement::new(105, prime).unwrap();
        let p1 = Point::new(Some(x1), Some(y1), a, b).unwrap();

        // Point 2: (17, 56)
        let x2 = FieldElement::new(17, prime).unwrap();
        let y2 = FieldElement::new(56, prime).unwrap();
        let p2 = Point::new(Some(x2), Some(y2), a, b).unwrap();

        // Expected result: (170, 142)
        let result = (p1 + p2).unwrap();
        assert_eq!(result.x.unwrap().value(), 170);
        assert_eq!(result.y.unwrap().value(), 142);
    }

    #[test]
    fn test_point_doubling() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();

        // Point: (192, 105)
        let x = FieldElement::new(192, prime).unwrap();
        let y = FieldElement::new(105, prime).unwrap();
        let p = Point::new(Some(x), Some(y), a, b).unwrap();

        // Double the point
        let result = (p.clone() + p).unwrap();

        // Expected result: (49, 71)
        assert_eq!(result.x.unwrap().value(), 49);
        assert_eq!(result.y.unwrap().value(), 71);
    }

    #[test]
    fn test_scalar_multiplication() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();

        // Point: (192, 105)
        let x = FieldElement::new(192, prime).unwrap();
        let y = FieldElement::new(105, prime).unwrap();
        let p = Point::new(Some(x), Some(y), a, b).unwrap();

        // Multiply by 2
        let result = (p * 2).unwrap();
        assert_eq!(result.x.unwrap().value(), 49);
        assert_eq!(result.y.unwrap().value(), 71);
    }

    #[test]
    fn test_point_negation() {
        let point = create_test_point();
        let neg_point = -point;

        assert_eq!(neg_point.x.unwrap().value(), 192);
        assert_eq!(neg_point.y.unwrap().value(), 223 - 105); // -105 mod 223
    }
}
