use std::ops::{Add, Div, Mul, Neg, Sub};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FieldError {
    #[error("Invalid field element: value must be between 0 and p-1")]
    InvalidElement,
    #[error("Mismatched fields: operations can only be performed on elements of the same field")]
    MismatchedFields,
    #[error("Division by zero")]
    DivisionByZero,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FieldElement {
    value: i64,
    prime: i64,
}

/// FieldElementの生成と操作を定義
impl FieldElement {
    pub fn new(value: i64, prime: i64) -> Result<Self, FieldError> {
        if prime <= 0 {
            return Err(FieldError::InvalidElement);
        }
        // 値を正規化
        let normalized_value = ((value % prime) + prime) % prime;
        Ok(Self {
            value: normalized_value,
            prime,
        })
    }

    /// FieldElementの値を取得
    pub fn value(&self) -> i64 {
        self.value
    }

    /// FieldElementの素数を取得
    pub fn prime(&self) -> i64 {
        self.prime
    }

    /// べき乗を計算
    pub fn pow(&self, exp: i64) -> Result<Self, FieldError> {
        let n = if exp >= 0 {
            exp
        } else {
            // Negative exponent: need to find multiplicative inverse first
            return self.pow(self.prime - 1 + exp);
        };

        let mut current = *self;
        let mut result = Self::new(1, self.prime)?;
        let mut e = n;

        while e > 0 {
            if e & 1 == 1 {
                result = result * current;
            }
            current = current * current;
            e >>= 1;
        }

        Ok(result)
    }

    /// 逆元を計算
    fn inv(&self) -> Result<Self, FieldError> {
        if self.value == 0 {
            return Err(FieldError::DivisionByZero);
        }

        let mut old_r = self.prime;
        let mut r = self.value;
        let mut old_s = 1;
        let mut s = 0;
        let mut old_t = 0;
        let mut t = 1;

        while r != 0 {
            let quotient = old_r / r;
            (old_r, r) = (r, old_r - quotient * r);
            (old_s, s) = (s, old_s - quotient * s);
            (old_t, t) = (t, old_t - quotient * t);
        }

        if old_r != 1 {
            return Err(FieldError::InvalidElement);
        }

        Self::new(old_t, self.prime)
    }
}

/// FieldElementに対する算術演算を実装
impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(
            self.prime, other.prime,
            "Cannot add elements of different fields"
        );
        Self::new((self.value + other.value) % self.prime, self.prime)
            .expect("Addition should never fail with valid elements")
    }
}

/// FieldElementに対する算術演算を実装（減算）
impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert_eq!(
            self.prime, other.prime,
            "Cannot subtract elements of different fields"
        );
        Self::new(
            (self.value - other.value + self.prime) % self.prime,
            self.prime,
        )
        .expect("Subtraction should never fail with valid elements")
    }
}

/// FieldElementに対する算術演算を実装（乗算）
impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        assert_eq!(
            self.prime, other.prime,
            "Cannot multiply elements of different fields"
        );
        Self::new((self.value * other.value) % self.prime, self.prime)
            .expect("Multiplication should never fail with valid elements")
    }
}

/// FieldElementに対する算術演算を実装（除算）
impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        assert_eq!(
            self.prime, other.prime,
            "Cannot divide elements of different fields"
        );
        let inverse = other.inv().expect("Division by zero");
        self * inverse
    }
}

/// FieldElementに対する算術演算を実装（単項マイナス）
impl Neg for FieldElement {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.value, self.prime).expect("Negation should never fail with valid elements")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_element_creation() {
        let element = FieldElement::new(7, 13).unwrap();
        assert_eq!(element.value(), 7);
        assert_eq!(element.prime(), 13);
    }

    #[test]
    fn test_field_element_addition() {
        let a = FieldElement::new(7, 13).unwrap();
        let b = FieldElement::new(12, 13).unwrap();
        let result = a + b;
        assert_eq!(result.value(), 6); // (7 + 12) % 13 = 6
    }

    #[test]
    fn test_field_element_subtraction() {
        let a = FieldElement::new(7, 13).unwrap();
        let b = FieldElement::new(12, 13).unwrap();
        let result = a - b;
        assert_eq!(result.value(), 8); // (7 - 12 + 13) % 13 = 8
    }

    #[test]
    fn test_field_element_multiplication() {
        let a = FieldElement::new(3, 13).unwrap();
        let b = FieldElement::new(12, 13).unwrap();
        let result = a * b;
        assert_eq!(result.value(), 10); // (3 * 12) % 13 = 10
    }

    #[test]
    fn test_field_element_division() {
        let a = FieldElement::new(3, 13).unwrap();
        let b = FieldElement::new(2, 13).unwrap();
        let result = a / b;
        assert_eq!(result.value(), 8); // 3 * 7 % 13 = 8 (where 7 is the multiplicative inverse of 2 mod 13)
    }

    #[test]
    fn test_field_element_power() {
        let base = FieldElement::new(3, 13).unwrap();
        let result = base.pow(3).unwrap();
        assert_eq!(result.value(), 1); // 3^3 % 13 = 1
    }
}
