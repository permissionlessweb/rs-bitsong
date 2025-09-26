//! Transaction builder and signer for Bitsong.
//!
//! This library provides support for building and signing transactions for the Bitsong blockchain,
//! with a focus on Bitsong's custom modules including cadence, fantoken, and smartaccount.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(html_root_url = "https://docs.rs/bitsong-rs/latest/")]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

extern crate alloc;

pub use tendermint;

pub mod cadence;
pub mod error;
pub mod fantoken;
pub mod smartaccount;

// Re-exports from cosmrs-like structure
pub use rs_bitsong_proto::types::cosmos::base::v1beta1::{Coin, DecCoin};
pub use tendermint::{account::Id as AccountId, chain::Id as ChainId, Hash};

pub use error::{Error, Result};

/// Bitsong-specific transaction building utilities
pub mod tx {
    pub use rs_bitsong_proto::types::cosmos::tx::signing::v1beta1::SignMode;
    pub use rs_bitsong_proto::types::cosmos::tx::v1beta1::{
        AuthInfo, Fee, ModeInfo, SignDoc, SignerInfo, Tx, TxBody, TxRaw,
    };
    pub use tendermint::block;
}

/// Re-export the proto types for convenience
pub use rs_bitsong_proto::types;
