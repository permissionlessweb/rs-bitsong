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

/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const GO_BITSONG_VERSION: &str = include_str!("types/BITSONG_COMMIT");

mod serde;
#[allow(deprecated, unused_imports, clippy::large_enum_variant)]
pub mod types;

pub mod shim;
pub mod traits;

pub use shim::{cosmwasm_to_proto_coins, try_proto_to_cosmwasm_coins};
