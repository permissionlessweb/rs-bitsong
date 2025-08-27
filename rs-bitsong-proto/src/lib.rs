// #![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cosmos/cosmos-rust/main/.images/cosmos.png"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(
    rustdoc::bare_urls,
    rustdoc::broken_intra_doc_links,
    clippy::derive_partial_eq_without_eq
)]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod traits;

pub use prost;
pub use tendermint_proto as tendermint;
pub use tendermint_proto::google::protobuf::{Any, Timestamp};
pub use cosmos_sdk_proto::cosmos;

/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const GO_BITSONG_VERSION: &str = include_str!("prost/rs-bitsong-proto/GO_BITSONG_COMMIT");

/// Cosmos protobuf definitions.
pub mod bitsong {
    /// Authentication of accounts and transactions.
    pub mod smartaccount {
        pub mod v1beta1 {
            include!("prost/rs-bitsong-proto/bitsong.smartaccount.v1beta1.rs");
        }
    }
    pub mod fantoken {
        pub mod v1beta1 {
            include!("prost/rs-bitsong-proto/bitsong.fantoken.v1beta1.rs");
        }
    }
    pub mod cadence {
        pub mod v1beta1 {
            include!("prost/rs-bitsong-proto/bitsong.cadence.v1.rs");
        }
    }
}