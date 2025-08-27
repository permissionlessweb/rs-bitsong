use rs_bitsong_derive::CosmwasmExt;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.EventIssue")]
pub struct EventIssue {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.EventDisableMint")]
pub struct EventDisableMint {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.EventMint")]
pub struct EventMint {
    #[prost(string, tag = "1")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub coin: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.EventBurn")]
pub struct EventBurn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub coin: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.EventSetAuthority")]
pub struct EventSetAuthority {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub old_authority: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub new_authority: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.EventSetMinter")]
pub struct EventSetMinter {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub old_minter: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub new_minter: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.EventSetUri")]
pub struct EventSetUri {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.Metadata")]
pub struct Metadata {
    /// name defines the name of the fantoken (eg: Kitty Punk)
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// symbol is the token symbol usually shown on exchanges (eg: KITTY)
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    /// URI to a document (on or off-chain) that contains additional
    /// information.Optional.
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// sdk.AccAddress allowed to set a new uri
    #[prost(string, tag = "4")]
    pub authority: ::prost::alloc::string::String,
}
/// FanToken defines a standard for the fungible token
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.FanToken")]
pub struct FanToken {
    /// denom represents the string name of the given denom unit (e.g ft<hash>).
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub max_supply: ::prost::alloc::string::String,
    /// sdk.AccAddress allowed to mint new fantoken
    #[prost(string, tag = "3")]
    pub minter: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub meta_data: ::core::option::Option<Metadata>,
}
/// Params defines fantoken module's parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.Params")]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub issue_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "2")]
    pub mint_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "3")]
    pub burn_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the fantoken module's genesis state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub fan_tokens: ::prost::alloc::vec::Vec<FanToken>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.UpdateFeesProposal")]
pub struct UpdateFeesProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub issue_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "4")]
    pub mint_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "5")]
    pub burn_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.UpdateFeesProposalWithDeposit")]
pub struct UpdateFeesProposalWithDeposit {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub issue_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub mint_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub burn_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub deposit: ::prost::alloc::string::String,
}
/// QueryFanTokenRequest is request type for the Query/FanToken RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.QueryFanTokenRequest")]
#[proto_query(
    path = "/bitsong.fantoken.v1beta1.Query/FanToken",
    response_type = QueryFanTokenResponse
)]
pub struct QueryFanTokenRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryFanTokenResponse is response type for the Query/FanToken RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.QueryFanTokenResponse")]
pub struct QueryFanTokenResponse {
    #[prost(message, optional, tag = "1")]
    pub fantoken: ::core::option::Option<FanToken>,
}
/// QueryFanTokensRequest is request type for the Query/FanTokens RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.QueryFanTokensRequest")]
#[proto_query(
    path = "/bitsong.fantoken.v1beta1.Query/FanTokens",
    response_type = QueryFanTokensResponse
)]
pub struct QueryFanTokensRequest {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryFanTokensResponse is response type for the Query/FanTokens RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.QueryFanTokensResponse")]
pub struct QueryFanTokensResponse {
    #[prost(message, repeated, tag = "1")]
    pub fantokens: ::prost::alloc::vec::Vec<FanToken>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryParametersRequest is request type for the Query/Parameters RPC method
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
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/bitsong.fantoken.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParametersResponse is response type for the Query/Parameters RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgIssue defines a message for issuing a new fan token
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.MsgIssue")]
pub struct MsgIssue {
    /// symbol which corresponds to the symbol of the fan token. It is a string and
    /// cannot change for the whole life of the fan token
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
    /// name which corresponds to the name of the fan token. It is a string and
    /// cannot change for the whole life of the fan token
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// max_supply that represents the maximum number of possible mintable tokens.
    /// It is an integer number, expressed in micro unit 10^6
    #[prost(string, tag = "3")]
    pub max_supply: ::prost::alloc::string::String,
    /// authority which is who can set a new uri metadata
    #[prost(string, tag = "4")]
    pub authority: ::prost::alloc::string::String,
    /// minter who is who can mint new fantoken and disable the minter process, the
    /// minter key also pay the gas fee
    #[prost(string, tag = "5")]
    pub minter: ::prost::alloc::string::String,
    /// URI which is the current uri of the fan token. It is a string can change
    /// during the fan token lifecycle thanks to the MsgEdit
    #[prost(string, tag = "6")]
    pub uri: ::prost::alloc::string::String,
}
/// MsgIssueResponse defines the MsgIssue response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.MsgIssueResponse")]
pub struct MsgIssueResponse {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// MsgDisableMint defines a message for disable the mint function
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.MsgDisableMint")]
pub struct MsgDisableMint {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub minter: ::prost::alloc::string::String,
}
/// MsgDisableMintResponse defines the MsgDisableMint response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.MsgDisableMintResponse")]
pub struct MsgDisableMintResponse {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// MsgMint defines a message for minting a new fan token
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.MsgMint")]
pub struct MsgMint {
    #[prost(string, tag = "1")]
    pub recipient: ::prost::alloc::string::String,
    /// coin mean the amount + denom, eg: 10000ftFADJID34MCDM
    #[prost(message, optional, tag = "2")]
    pub coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub minter: ::prost::alloc::string::String,
}
/// MsgMintResponse defines the MsgMint response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.MsgMintResponse")]
pub struct MsgMintResponse {
    #[prost(string, tag = "1")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgBurn defines a message for burning some fan tokens
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.MsgBurn")]
pub struct MsgBurn {
    /// coin mean the amount + denom, eg: 10000ftFADJID34MCDM
    #[prost(message, optional, tag = "1")]
    pub coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
}
/// MsgBurnResponse defines the MsgBurn response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.MsgBurnResponse")]
pub struct MsgBurnResponse {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSetMinter defines a message for changing the fan token minter address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.MsgSetMinter")]
pub struct MsgSetMinter {
    /// denom the fan token denom
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// old_minter, the actual minter
    #[prost(string, tag = "2")]
    pub old_minter: ::prost::alloc::string::String,
    /// new_minter, the new fan token minter
    #[prost(string, tag = "3")]
    pub new_minter: ::prost::alloc::string::String,
}
/// MsgSetMinterResponse defines the MsgTransferAuthority response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.MsgSetMinterResponse")]
pub struct MsgSetMinterResponse {
    /// denom the fan token denom
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// old_minter, the actual minter
    #[prost(string, tag = "2")]
    pub old_minter: ::prost::alloc::string::String,
    /// new_minter, the new fan token minter
    #[prost(string, tag = "3")]
    pub new_minter: ::prost::alloc::string::String,
}
/// MsgSetAuthority defines a message for changing the fan token minter address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.MsgSetAuthority")]
pub struct MsgSetAuthority {
    /// denom the fan token denom
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// old_authority, the actual metadata authority
    #[prost(string, tag = "2")]
    pub old_authority: ::prost::alloc::string::String,
    /// new_authority, the new fan token metadata authority
    #[prost(string, tag = "3")]
    pub new_authority: ::prost::alloc::string::String,
}
/// MsgSetAuthorityResponse defines the MsgTransferAuthority response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.MsgSetAuthorityResponse")]
pub struct MsgSetAuthorityResponse {
    /// denom the fan token denom
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// old_authority, the actual metadata authority
    #[prost(string, tag = "2")]
    pub old_authority: ::prost::alloc::string::String,
    /// new_authority, the new fan token metadata authority
    #[prost(string, tag = "3")]
    pub new_authority: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.MsgSetUri")]
pub struct MsgSetUri {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/bitsong.fantoken.v1beta1.MsgSetUriResponse")]
pub struct MsgSetUriResponse {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
pub struct FantokenQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> FantokenQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn fan_token(
        &self,
        denom: ::prost::alloc::string::String,
    ) -> Result<QueryFanTokenResponse, cosmwasm_std::StdError> {
        QueryFanTokenRequest { denom }.query(self.querier)
    }
    pub fn fan_tokens(
        &self,
        authority: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryFanTokensResponse, cosmwasm_std::StdError> {
        QueryFanTokensRequest {
            authority,
            pagination,
        }
        .query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
}
