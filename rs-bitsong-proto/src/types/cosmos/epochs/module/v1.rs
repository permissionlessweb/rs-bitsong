use rs_bitsong_derive::CosmwasmExt;
/// Module is the config object of the epochs module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.epochs.module.v1.Module")]
pub struct Module {}
