use rs_bitsong_derive::CosmwasmExt;
/// PubKey is an bls12381 public key for aggregated
/// It's needed for Any serialization and SDK compatibility.
/// It must not be used in a non Tendermint key context because it doesn't implement
/// ADR-28. Nevertheless, you will like to use bls12381 in app user level
/// then you must create a new proto message and follow ADR-28 for Address construction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.crypto.bls12381.PubKey")]
pub struct PubKey {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// PrivKey defines a bls12381 private key.
/// NOTE: bls12381 keys must not be used in SDK apps except in a tendermint validator context.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.crypto.bls12381.PrivKey")]
pub struct PrivKey {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
