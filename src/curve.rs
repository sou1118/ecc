use crate::field::FieldElement;
use crate::point::Point;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CurveError {
    #[error("Invalid curve parameters")]
    InvalidParameters,
    #[error("Point generation failed")]
    PointGenerationFailed,
}

#[derive(Debug, Clone)]
pub struct Curve {
    pub a: FieldElement,
    pub b: FieldElement,
    pub prime: i64,
}

/// 曲線の定義
impl Curve {
    pub fn new(a: i64, b: i64, prime: i64) -> Result<Self, CurveError> {
        let a = FieldElement::new(a, prime).map_err(|_| CurveError::InvalidParameters)?;
        let b = FieldElement::new(b, prime).map_err(|_| CurveError::InvalidParameters)?;

        // 4a³ + 27b² ≠ 0 の確認
        let a_cubed = a * a * a * FieldElement::new(4, prime).unwrap();
        let b_squared = b * b * FieldElement::new(27, prime).unwrap();
        if (a_cubed + b_squared).value() == 0 {
            return Err(CurveError::InvalidParameters);
        }

        Ok(Self { a, b, prime })
    }

    /// 指定された座標にある点を生成
    pub fn point(&self, x: i64, y: i64) -> Result<Point, CurveError> {
        let x = FieldElement::new(x, self.prime).map_err(|_| CurveError::PointGenerationFailed)?;
        let y = FieldElement::new(y, self.prime).map_err(|_| CurveError::PointGenerationFailed)?;
        Point::new(Some(x), Some(y), self.a, self.b).map_err(|_| CurveError::PointGenerationFailed)
    }

    /// 無限遠点を生成
    pub fn infinity_point(&self) -> Point {
        Point::new(None, None, self.a, self.b).unwrap()
    }

    // 点の位数を計算
    pub fn point_order(&self, point: &Point) -> Result<i64, CurveError> {
        let mut current = point.clone();
        for n in 1..=self.prime {
            match current.clone() + point.clone() {
                Ok(next) => {
                    if next.is_infinity() {
                        return Ok(n + 1);
                    }
                    current = next;
                }
                Err(_) => return Err(CurveError::PointGenerationFailed),
            }
        }
        Err(CurveError::PointGenerationFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_creation() {
        // y² = x³ + 7 over F223
        let curve = Curve::new(0, 7, 223).unwrap();
        assert_eq!(curve.a.value(), 0);
        assert_eq!(curve.b.value(), 7);
        assert_eq!(curve.prime, 223);
    }

    #[test]
    fn test_point_creation_on_curve() {
        let curve = Curve::new(0, 7, 223).unwrap();
        let point = curve.point(192, 105).unwrap();
        assert_eq!(point.x.unwrap().value(), 192);
        assert_eq!(point.y.unwrap().value(), 105);
    }

    #[test]
    fn test_invalid_point() {
        let curve = Curve::new(0, 7, 223).unwrap();
        assert!(curve.point(200, 119).is_err()); // This point is not on the curve
    }

    #[test]
    fn test_infinity_point() {
        let curve = Curve::new(0, 7, 223).unwrap();
        let point = curve.infinity_point();
        assert!(point.is_infinity());
    }

    #[test]
    fn test_point_order() {
        let curve = Curve::new(0, 7, 223).unwrap();
        let point = curve.point(192, 105).unwrap();

        // 計算した位数が正しいことを確認
        let order = curve.point_order(&point).unwrap();
        let mul_result = (point * order).unwrap();
        assert!(mul_result.is_infinity());
    }
}
