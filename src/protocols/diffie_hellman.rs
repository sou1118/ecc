use super::{generate_private_key, ProtocolError};
use crate::curve::Curve;
use crate::point::Point;

pub struct DiffieHellman {
    private_key: i64,
    public_key: Point,
}

/// Diffie-Hellman鍵交換の実装
impl DiffieHellman {
    /// 新しいDiffie-Hellmanインスタンスを作成
    pub fn new(curve: Curve, generator: Point) -> Result<Self, ProtocolError> {
        let order = curve
            .point_order(&generator)
            .map_err(|_| ProtocolError::InvalidParameters)?;

        let private_key = generate_private_key(order);
        let public_key =
            (generator.clone() * private_key).map_err(|_| ProtocolError::OperationFailed)?;

        Ok(Self {
            private_key,
            public_key,
        })
    }

    /// 公開鍵を取得
    pub fn public_key(&self) -> &Point {
        &self.public_key
    }

    /// 共有鍵を計算
    pub fn compute_shared_secret(&self, other_public: &Point) -> Result<Point, ProtocolError> {
        (other_public.clone() * self.private_key).map_err(|_| ProtocolError::OperationFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diffie_hellman_key_exchange() {
        // 小さな素数を使用してテスト
        let curve = Curve::new(0, 7, 223).unwrap();
        let generator = curve.point(15, 86).unwrap();

        // Alice と Bob のインスタンスを作成
        let alice = DiffieHellman::new(curve.clone(), generator.clone()).unwrap();
        let bob = DiffieHellman::new(curve.clone(), generator.clone()).unwrap();

        // 共有鍵を計算
        let alice_shared = alice.compute_shared_secret(bob.public_key()).unwrap();
        let bob_shared = bob.compute_shared_secret(alice.public_key()).unwrap();

        // 両者の共有鍵が一致することを確認
        assert_eq!(alice_shared.x, bob_shared.x);
        assert_eq!(alice_shared.y, bob_shared.y);
    }

    #[test]
    fn test_different_private_keys_same_shared_secret() {
        let curve = Curve::new(0, 7, 223).unwrap();
        let generator = curve.point(15, 86).unwrap();

        // 手動で異なる秘密鍵を設定してテスト
        let dh1 = DiffieHellman {
            private_key: 7, // 明示的に異なる値を使用
            public_key: (generator.clone() * 7).unwrap(),
        };

        let dh2 = DiffieHellman {
            private_key: 13, // 明示的に異なる値を使用
            public_key: (generator.clone() * 13).unwrap(),
        };

        // 秘密鍵が異なることを確認
        assert_ne!(dh1.private_key, dh2.private_key);

        // 共有鍵を計算
        let shared1 = dh1.compute_shared_secret(dh2.public_key()).unwrap();
        let shared2 = dh2.compute_shared_secret(dh1.public_key()).unwrap();

        // 共有鍵が同じになることを確認
        assert_eq!(shared1.x, shared2.x);
        assert_eq!(shared1.y, shared2.y);
    }
}
