use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZkLedgerError {
    #[error("Wallet not found: {0}")]
    WalletNotFound(String),

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Invalid proof")]
    InvalidProof,

    #[error("Note already spent")]
    NoteAlreadySpent,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid amount")]
    InvalidAmount,

    #[error("Invalid key format")]
    InvalidKeyFormat,
} 