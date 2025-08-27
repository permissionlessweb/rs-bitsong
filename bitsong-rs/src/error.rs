//! Error types for the bitsong-rs library.
use thiserror::Error;

/// Error type for bitsong-rs operations
#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Error {
    /// Invalid coin denomination
    #[error("invalid coin denomination: {0}")]
    InvalidDenom(String),

    /// Invalid address format
    #[error("invalid address: {0}")]
    InvalidAddress(String),

    /// Invalid message type
    #[error("invalid message type: {0}")]
    InvalidMessage(String),

    /// Encoding/decoding errors
    #[error("encoding error: {0}")]
    Encoding(String),

    /// Protobuf errors
    #[error("protobuf error: {0}")]
    Protobuf(String),

    /// Signature errors
    #[error("signature error: {0}")]
    Signature(String),

    /// Transaction building errors
    #[error("transaction error: {0}")]
    Transaction(String),

    /// Cadence module specific errors
    #[error("cadence error: {0}")]
    Cadence(String),

    /// Fantoken module specific errors
    #[error("fantoken error: {0}")]
    Fantoken(String),

    /// Smart account module specific errors
    #[error("smartaccount error: {0}")]
    SmartAccount(String),
}

/// Common error type for the library
pub type Result<T, E = Error> = core::result::Result<T, E>;
