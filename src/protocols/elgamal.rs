use super::{generate_private_key, ProtocolError};
use crate::curve::Curve;
use crate::point::Point;

pub struct ElGamal {
    curve: Curve,
    generator: Point,
    private_key: i64,
    public_key: Point,
}

#[derive(Debug, Clone)]
pub struct Ciphertext {
    pub c1: Point, // rG
    pub c2: Point, // M + rB
}

/// ElGamal暗号の実装
impl ElGamal {
    /// 新しいElGamalインスタンスを作成
    pub fn new(curve: Curve, generator: Point) -> Result<Self, ProtocolError> {
        let order = curve
            .point_order(&generator)
            .map_err(|_| ProtocolError::InvalidParameters)?;

        let private_key = generate_private_key(order);
        let public_key =
            (generator.clone() * private_key).map_err(|_| ProtocolError::OperationFailed)?;

        Ok(Self {
            curve,
            generator,
            private_key,
            public_key,
        })
    }

    pub fn public_key(&self) -> &Point {
        &self.public_key
    }

    /// メッセージを暗号化
    pub fn encrypt(&self, message: &Point, r: Option<i64>) -> Result<Ciphertext, ProtocolError> {
        let order = self
            .curve
            .point_order(&self.generator)
            .map_err(|_| ProtocolError::InvalidParameters)?;

        // ランダムなrを生成
        let r = r.unwrap_or_else(|| generate_private_key(order));

        // c1 = rG を計算
        let c1 = (self.generator.clone() * r).map_err(|_| ProtocolError::OperationFailed)?;

        // c2 = M + rB を計算
        let rb = (self.public_key.clone() * r).map_err(|_| ProtocolError::OperationFailed)?;
        let c2 = (message.clone() + rb).map_err(|_| ProtocolError::OperationFailed)?;

        Ok(Ciphertext { c1, c2 })
    }

    /// 暗号文を復号
    pub fn decrypt(&self, ciphertext: &Ciphertext) -> Result<Point, ProtocolError> {
        // -kC1 を計算
        let neg_kc1 = (ciphertext.c1.clone() * self.private_key)
            .map_err(|_| ProtocolError::OperationFailed)
            .map(|p| -p)?;

        // M = C2 + (-kC1) を計算
        (ciphertext.c2.clone() + neg_kc1).map_err(|_| ProtocolError::OperationFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_curve() -> (Curve, Point) {
        let curve = Curve::new(0, 7, 223).unwrap(); // y² = x³ + 7
        let generator = curve.point(15, 86).unwrap(); // 適当な生成点
        (curve, generator)
    }

    #[test]
    fn test_encryption_decryption() {
        let (curve, generator) = setup_test_curve();

        // Alice のインスタンスを作成（受信者）
        let alice = ElGamal::new(curve.clone(), generator.clone()).unwrap();

        // メッセージを作成（曲線上の点として）
        let message = curve.point(192, 105).unwrap();

        // 暗号化（rの値を固定してテストを決定的にする）
        let ciphertext = alice.encrypt(&message, Some(17)).unwrap();

        // 復号
        let decrypted = alice.decrypt(&ciphertext).unwrap();

        // 元のメッセージと一致することを確認
        assert_eq!(decrypted.x, message.x);
        assert_eq!(decrypted.y, message.y);
    }

    #[test]
    fn test_different_random_values() {
        let (curve, generator) = setup_test_curve();
        let alice = ElGamal::new(curve.clone(), generator.clone()).unwrap();
        let message = curve.point(192, 105).unwrap();

        // 異なる r の値で暗号化
        let ciphertext1 = alice.encrypt(&message, Some(17)).unwrap();
        let ciphertext2 = alice.encrypt(&message, Some(19)).unwrap();

        // 暗号文が異なることを確認
        assert_ne!(ciphertext1.c1.x, ciphertext2.c1.x);
        assert_ne!(ciphertext1.c2.x, ciphertext2.c2.x);

        // 復号結果は同じになることを確認
        let decrypted1 = alice.decrypt(&ciphertext1).unwrap();
        let decrypted2 = alice.decrypt(&ciphertext2).unwrap();
        assert_eq!(decrypted1.x, decrypted2.x);
        assert_eq!(decrypted1.y, decrypted2.y);
    }

    #[test]
    fn test_encryption_homomorphism() {
        let (curve, generator) = setup_test_curve();
        let alice = ElGamal::new(curve.clone(), generator.clone()).unwrap();

        // 2つのメッセージ
        let m1 = curve.point(192, 105).unwrap();
        let m2 = curve.point(17, 56).unwrap();

        // それぞれ暗号化
        let c1 = alice.encrypt(&m1, Some(17)).unwrap();
        let c2 = alice.encrypt(&m2, Some(19)).unwrap();

        // 暗号文の加法準同型性をテスト
        let sum_encrypted = Ciphertext {
            c1: (c1.c1.clone() + c2.c1.clone()).unwrap(),
            c2: (c1.c2.clone() + c2.c2.clone()).unwrap(),
        };

        let sum_decrypted = alice.decrypt(&sum_encrypted).unwrap();
        let expected_sum = (m1 + m2).unwrap();

        assert_eq!(sum_decrypted.x, expected_sum.x);
        assert_eq!(sum_decrypted.y, expected_sum.y);
    }
}
