use rs_bitsong_derive::CosmwasmExt;
/// This object is used to store the contract address and the
/// jail status of the contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.cadence.v1.CadenceContract")]
pub struct CadenceContract {
    /// The address of the contract.
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// The jail status of the contract.
    #[prost(bool, tag = "2")]
    pub is_jailed: bool,
}
/// GenesisState - initial state of module
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
#[proto_message(type_url = "/bitsong.cadence.v1.GenesisState")]
pub struct GenesisState {
    /// Params of this module
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// Params defines the set of module parameters.
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
#[proto_message(type_url = "/bitsong.cadence.v1.Params")]
pub struct Params {
    /// contract_gas_limit defines the maximum amount of gas that can be used by a contract.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub contract_gas_limit: u64,
}
/// QueryCadenceContracts is the request type to get all contracts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.cadence.v1.QueryCadenceContracts")]
#[proto_query(
    path = "/bitsong.cadence.v1.Query/CadenceContracts",
    response_type = QueryCadenceContractsResponse
)]
pub struct QueryCadenceContracts {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryCadenceContractsResponse is the response type for the Query/CadenceContracts RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.cadence.v1.QueryCadenceContractsResponse")]
pub struct QueryCadenceContractsResponse {
    /// cadence_contracts are the cadence contract s.
    #[prost(message, repeated, tag = "1")]
    pub cadence_contracts: ::prost::alloc::vec::Vec<CadenceContract>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryCadenceContract is the request type to get a single contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.cadence.v1.QueryCadenceContract")]
#[proto_query(
    path = "/bitsong.cadence.v1.Query/CadenceContract",
    response_type = QueryCadenceContractResponse
)]
pub struct QueryCadenceContract {
    /// contract_address is the address of the contract to query.
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
}
/// QueryCadenceContractResponse is the response type for the Query/CadenceContract RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.cadence.v1.QueryCadenceContractResponse")]
pub struct QueryCadenceContractResponse {
    /// contract is the cadence contract .
    #[prost(message, optional, tag = "1")]
    pub cadence_contract: ::core::option::Option<CadenceContract>,
}
/// QueryParams is the request type to get all module params.
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
#[proto_message(type_url = "/bitsong.cadence.v1.QueryParamsRequest")]
#[proto_query(
    path = "/bitsong.cadence.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryCadenceContractsResponse is the response type for the Query/CadenceContracts RPC method.
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
#[proto_message(type_url = "/bitsong.cadence.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgRegisterCadenceContract is the Msg/RegisterCadenceContract request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.cadence.v1.MsgRegisterCadenceContract")]
pub struct MsgRegisterCadenceContract {
    /// The address of the sender.
    #[prost(string, tag = "1")]
    pub sender_address: ::prost::alloc::string::String,
    /// The address of the contract to register.
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
}
/// MsgRegisterCadenceContractResponse defines the response structure for executing a
/// MsgRegisterCadenceContract message.
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
#[proto_message(type_url = "/bitsong.cadence.v1.MsgRegisterCadenceContractResponse")]
pub struct MsgRegisterCadenceContractResponse {}
/// MsgUnregisterCadenceContract is the Msg/UnregisterCadenceContract request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.cadence.v1.MsgUnregisterCadenceContract")]
pub struct MsgUnregisterCadenceContract {
    /// The address of the sender.
    #[prost(string, tag = "1")]
    pub sender_address: ::prost::alloc::string::String,
    /// The address of the contract to unregister.
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
}
/// MsgUnregisterCadenceContractResponse defines the response structure for executing a
/// MsgUnregisterCadenceContract message.
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
#[proto_message(type_url = "/bitsong.cadence.v1.MsgUnregisterCadenceContractResponse")]
pub struct MsgUnregisterCadenceContractResponse {}
/// MsgUnjailCadenceContract is the Msg/UnjailCadenceContract request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.cadence.v1.MsgUnjailCadenceContract")]
pub struct MsgUnjailCadenceContract {
    /// The address of the sender.
    #[prost(string, tag = "1")]
    pub sender_address: ::prost::alloc::string::String,
    /// The address of the contract to unjail.
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
}
/// MsgUnjailCadenceContractResponse defines the response structure for executing a
/// MsgUnjailCadenceContract message.
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
#[proto_message(type_url = "/bitsong.cadence.v1.MsgUnjailCadenceContractResponse")]
pub struct MsgUnjailCadenceContractResponse {}
/// MsgUpdateParams is the Msg/UpdateParams request type.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.cadence.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/cadence parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: cosmos-sdk 0.47
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
#[proto_message(type_url = "/bitsong.cadence.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct CadenceQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> CadenceQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn cadence_contracts(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryCadenceContractsResponse, cosmwasm_std::StdError> {
        QueryCadenceContracts { pagination }.query(self.querier)
    }
    pub fn cadence_contract(
        &self,
        contract_address: ::prost::alloc::string::String,
    ) -> Result<QueryCadenceContractResponse, cosmwasm_std::StdError> {
        QueryCadenceContract { contract_address }.query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
}
