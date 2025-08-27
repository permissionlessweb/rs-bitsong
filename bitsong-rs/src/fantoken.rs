//! Fantoken module support for the Bitsong blockchain.
//!
//! This module provides support for interacting with Bitsong's Fantoken module,
//! which handles fan token creation, minting, burning, and management.

use crate::types::{
    bitsong::fantoken::v1beta1::{
        FanToken, Metadata, MsgBurn, MsgDisableMint, MsgIssue, MsgMint, MsgSetAuthority,
        MsgSetMinter, MsgSetUri, Params, QueryFanTokenRequest, QueryFanTokenResponse,
        QueryFanTokensRequest, QueryFanTokensResponse,
        QueryParamsRequest as QueryFantokenParamsRequest,
        QueryParamsResponse as QueryFantokenParamsResponse,
    },
    cosmos::base::v1beta1::Coin,
};

use crate::{Error, Result};
use serde::{Deserialize, Serialize};

pub use crate::types::bitsong::fantoken::v1beta1::{
    FanToken as ProtoFanToken, Metadata as ProtoMetadata, Params as ProtoParams,
};

/// Fantoken metadata
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FantokenMetadata {
    /// Name of the fantoken (e.g., "Kitty Punk")
    pub name: String,
    /// Symbol is the token symbol usually shown on exchanges (e.g., "KITTY")
    pub symbol: String,
    /// URI to a document containing additional information (optional)
    pub uri: String,
    /// Address allowed to set a new URI
    pub authority: String,
}

impl From<Metadata> for FantokenMetadata {
    fn from(metadata: Metadata) -> Self {
        Self {
            name: metadata.name,
            symbol: metadata.symbol,
            uri: metadata.uri,
            authority: metadata.authority,
        }
    }
}

impl From<FantokenMetadata> for Metadata {
    fn from(metadata: FantokenMetadata) -> Self {
        Self {
            name: metadata.name,
            symbol: metadata.symbol,
            uri: metadata.uri,
            authority: metadata.authority,
        }
    }
}

/// Fantoken information
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FantokenInfo {
    /// The denomination of the fantoken
    pub denom: String,
    /// Maximum supply allowed for minting
    pub max_supply: String,
    /// Current minter address
    pub minter: String,
    /// Metadata for the fantoken
    pub metadata: FantokenMetadata,
}

impl From<FanToken> for FantokenInfo {
    fn from(fantoken: FanToken) -> Self {
        Self {
            denom: fantoken.denom,
            max_supply: fantoken.max_supply,
            minter: fantoken.minter,
            metadata: fantoken.meta_data.unwrap_or_default().into(),
        }
    }
}

impl From<FantokenInfo> for FanToken {
    fn from(info: FantokenInfo) -> Self {
        Self {
            denom: info.denom,
            max_supply: info.max_supply,
            minter: info.minter,
            meta_data: Some(info.metadata.into()),
        }
    }
}

/// Fantoken module parameters
#[derive(Clone, Debug, PartialEq)]
pub struct FantokenParams {
    /// Fee for issuing a fantoken
    pub issue_fee: Option<Coin>,
    /// Fee for minting fantoken
    pub mint_fee: Option<Coin>,
    /// Fee for burning fantoken
    pub burn_fee: Option<Coin>,
}

impl From<Params> for FantokenParams {
    fn from(params: Params) -> Self {
        Self {
            issue_fee: params.issue_fee,
            mint_fee: params.mint_fee,
            burn_fee: params.burn_fee,
        }
    }
}

impl From<FantokenParams> for Params {
    fn from(params: FantokenParams) -> Self {
        Self {
            issue_fee: params.issue_fee,
            mint_fee: params.mint_fee,
            burn_fee: params.burn_fee,
        }
    }
}

/// Message to issue a new fantoken
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MsgIssueFantoken {
    /// The symbol of the fantoken
    pub symbol: String,
    /// The name of the fantoken
    pub name: String,
    /// Maximum supply for minting
    pub max_supply: String,
    /// URI for additional information
    pub uri: String,
    /// Authority address
    pub authority: String,
    /// Minter address
    pub minter: String,
}

impl From<MsgIssueFantoken> for MsgIssue {
    fn from(msg: MsgIssueFantoken) -> Self {
        Self {
            symbol: msg.symbol,
            name: msg.name,
            max_supply: msg.max_supply,
            uri: msg.uri,
            authority: msg.authority,
            minter: msg.minter,
        }
    }
}

impl TryFrom<MsgIssue> for MsgIssueFantoken {
    type Error = Error;

    fn try_from(proto: MsgIssue) -> Result<Self> {
        Ok(Self {
            symbol: proto.symbol,
            name: proto.name,
            max_supply: proto.max_supply,
            uri: proto.uri,
            authority: proto.authority,
            minter: proto.minter,
        })
    }
}

/// Message to disable minting for a fantoken
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MsgDisableFantokenMint {
    /// The denomination of the fantoken
    pub denom: String,
    /// The minter address
    pub minter: String,
}

impl From<MsgDisableFantokenMint> for MsgDisableMint {
    fn from(msg: MsgDisableFantokenMint) -> Self {
        Self {
            denom: msg.denom,
            minter: msg.minter,
        }
    }
}

impl TryFrom<MsgDisableMint> for MsgDisableFantokenMint {
    type Error = Error;

    fn try_from(proto: MsgDisableMint) -> Result<Self> {
        Ok(Self {
            denom: proto.denom,
            minter: proto.minter,
        })
    }
}

/// Message to mint fantoken
#[derive(Clone, Debug, PartialEq)]
pub struct MsgMintFantoken {
    /// The recipient address
    pub recipient: String,
    /// The coin to mint
    pub coin: Option<Coin>,
    /// The minter address
    pub minter: String,
}

impl From<MsgMintFantoken> for MsgMint {
    fn from(msg: MsgMintFantoken) -> Self {
        Self {
            recipient: msg.recipient,
            coin: msg.coin,
            minter: msg.minter,
        }
    }
}

impl TryFrom<MsgMint> for MsgMintFantoken {
    type Error = Error;

    fn try_from(proto: MsgMint) -> Result<Self> {
        Ok(Self {
            recipient: proto.recipient,
            coin: proto.coin,
            minter: proto.minter,
        })
    }
}

/// Message to burn fantoken
#[derive(Clone, Debug, PartialEq)]
pub struct MsgBurnFantoken {
    /// The coin to burn
    pub coin: Option<Coin>,
    /// The sender address
    pub sender: String,
}

impl From<MsgBurnFantoken> for MsgBurn {
    fn from(msg: MsgBurnFantoken) -> Self {
        Self {
            coin: msg.coin,
            sender: msg.sender,
        }
    }
}

impl TryFrom<MsgBurn> for MsgBurnFantoken {
    type Error = Error;

    fn try_from(proto: MsgBurn) -> Result<Self> {
        Ok(Self {
            coin: proto.coin,
            sender: proto.sender,
        })
    }
}

/// Message to set fantoken authority
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MsgSetFantokenAuthority {
    /// The denomination of the fantoken
    pub denom: String,
    /// The old authority address
    pub old_authority: String,
    /// The new authority address
    pub new_authority: String,
}

impl From<MsgSetFantokenAuthority> for MsgSetAuthority {
    fn from(msg: MsgSetFantokenAuthority) -> Self {
        Self {
            denom: msg.denom,
            old_authority: msg.old_authority,
            new_authority: msg.new_authority,
        }
    }
}

impl TryFrom<MsgSetAuthority> for MsgSetFantokenAuthority {
    type Error = Error;

    fn try_from(proto: MsgSetAuthority) -> Result<Self> {
        Ok(Self {
            denom: proto.denom,
            old_authority: proto.old_authority,
            new_authority: proto.new_authority,
        })
    }
}

/// Message to set fantoken minter
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MsgSetFantokenMinter {
    /// The denomination of the fantoken
    pub denom: String,
    /// The old minter address
    pub old_minter: String,
    /// The new minter address
    pub new_minter: String,
}

impl From<MsgSetFantokenMinter> for MsgSetMinter {
    fn from(msg: MsgSetFantokenMinter) -> Self {
        Self {
            denom: msg.denom,
            old_minter: msg.old_minter,
            new_minter: msg.new_minter,
        }
    }
}

impl TryFrom<MsgSetMinter> for MsgSetFantokenMinter {
    type Error = Error;

    fn try_from(proto: MsgSetMinter) -> Result<Self> {
        Ok(Self {
            denom: proto.denom,
            old_minter: proto.old_minter,
            new_minter: proto.new_minter,
        })
    }
}

/// Message to set fantoken URI
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MsgSetFantokenUri {
    /// The denomination of the fantoken
    pub denom: String,
    /// The authority address
    pub authority: String,
    /// The new URI
    pub uri: String,
}

impl From<MsgSetFantokenUri> for MsgSetUri {
    fn from(msg: MsgSetFantokenUri) -> Self {
        Self {
            denom: msg.denom,
            authority: msg.authority,
            uri: msg.uri,
        }
    }
}

impl TryFrom<MsgSetUri> for MsgSetFantokenUri {
    type Error = Error;

    fn try_from(proto: MsgSetUri) -> Result<Self> {
        Ok(Self {
            denom: proto.denom,
            authority: proto.authority,
            uri: proto.uri,
        })
    }
}

/// Query request for a specific fantoken
pub struct QueryFantokenRequest {
    /// The denomination of the fantoken to query
    pub denom: String,
}

impl From<QueryFantokenRequest> for QueryFanTokenRequest {
    fn from(req: QueryFantokenRequest) -> Self {
        Self { denom: req.denom }
    }
}

impl From<QueryFanTokenRequest> for QueryFantokenRequest {
    fn from(proto: QueryFanTokenRequest) -> Self {
        Self { denom: proto.denom }
    }
}

/// Query request for all fantokens
pub struct QueryFantokensRequest {
    /// Authority filter (optional)
    pub authority: String,
    /// Pagination parameters
    pub pagination: Option<crate::types::cosmos::base::query::v1beta1::PageRequest>,
}

impl From<QueryFantokensRequest> for QueryFanTokensRequest {
    fn from(req: QueryFantokensRequest) -> Self {
        Self {
            authority: req.authority,
            pagination: req.pagination,
        }
    }
}

impl From<QueryFanTokensRequest> for QueryFantokensRequest {
    fn from(proto: QueryFanTokensRequest) -> Self {
        Self {
            authority: proto.authority,
            pagination: proto.pagination,
        }
    }
}

/// Query response for a specific fantoken
pub struct QueryFantokenResponse {
    /// The fantoken information
    pub fantoken: Option<FantokenInfo>,
}

impl From<QueryFanTokenResponse> for QueryFantokenResponse {
    fn from(proto: QueryFanTokenResponse) -> Self {
        Self {
            fantoken: proto.fantoken.map(|ft| ft.into()),
        }
    }
}

impl From<QueryFantokenResponse> for QueryFanTokenResponse {
    fn from(resp: QueryFantokenResponse) -> Self {
        Self {
            fantoken: resp.fantoken.map(|ft| ft.into()),
        }
    }
}

/// Query response for all fantokens
pub struct QueryFantokensResponse {
    /// List of fantokens
    pub fantokens: Vec<FantokenInfo>,
    /// Pagination information
    pub pagination: Option<crate::types::cosmos::base::query::v1beta1::PageResponse>,
}

impl From<QueryFanTokensResponse> for QueryFantokensResponse {
    fn from(proto: QueryFanTokensResponse) -> Self {
        Self {
            fantokens: proto.fantokens.into_iter().map(|ft| ft.into()).collect(),
            pagination: proto.pagination,
        }
    }
}

impl From<QueryFantokensResponse> for QueryFanTokensResponse {
    fn from(resp: QueryFantokensResponse) -> Self {
        Self {
            fantokens: resp.fantokens.into_iter().map(|ft| ft.into()).collect(),
            pagination: resp.pagination,
        }
    }
}

/// Query request for fantoken module parameters
pub struct QueryFantokenParams {}

impl From<QueryFantokenParams> for QueryFantokenParamsRequest {
    fn from(_: QueryFantokenParams) -> Self {
        Self {}
    }
}

impl From<QueryFantokenParamsRequest> for QueryFantokenParams {
    fn from(_: QueryFantokenParamsRequest) -> Self {
        Self {}
    }
}

/// Query response for fantoken module parameters
pub struct QueryFantokenParamsResp {
    /// The module parameters
    pub params: Option<FantokenParams>,
}

impl From<QueryFantokenParamsResponse> for QueryFantokenParamsResp {
    fn from(proto: QueryFantokenParamsResponse) -> Self {
        Self {
            params: proto.params.map(|p| p.into()),
        }
    }
}

impl From<QueryFantokenParamsResp> for QueryFantokenParamsResponse {
    fn from(resp: QueryFantokenParamsResp) -> Self {
        Self {
            params: resp.params.map(|p| p.into()),
        }
    }
}
