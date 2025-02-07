pub mod diffie_hellman;
pub mod elgamal;

use rand::Rng;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Invalid parameters")]
    InvalidParameters,
    #[error("Operation failed")]
    OperationFailed,
}

/// 鍵生成のためのヘルパー関数
pub(crate) fn generate_private_key(order: i64) -> i64 {
    let mut rng = rand::rng();
    rng.random_range(1..order)
}
