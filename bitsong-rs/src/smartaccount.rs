//! Smart Account module support for the Bitsong blockchain.
//!
//! This module provides support for interacting with Bitsong's Smart Account module,
//! which handles account authentication and smart account functionality.

use crate::types::bitsong::smartaccount::v1beta1::{
    AccountAuthenticator, AuthenticatorData, MsgAddAuthenticator, MsgRemoveAuthenticator,
    MsgSetActiveState, Params, QueryParamsRequest, QueryParamsResponse,
};

use crate::{Error, Result};
use serde::{Deserialize, Serialize};

pub use crate::types::bitsong::smartaccount::v1beta1::{
    AccountAuthenticator as ProtoAccountAuthenticator, AuthenticatorData as ProtoAuthenticatorData,
    Params as ProtoParams,
};

/// Smart Account module parameters
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SmartAccountParams {
    /// Maximum amount of gas for unauthenticated transactions
    pub maximum_unauthenticated_gas: u64,
    /// Whether smart account functionality is active
    pub is_smart_account_active: bool,
    /// List of addresses allowed to control circuit breaker
    pub circuit_breaker_controllers: Vec<String>,
}

impl From<Params> for SmartAccountParams {
    fn from(params: Params) -> Self {
        Self {
            maximum_unauthenticated_gas: params.maximum_unauthenticated_gas,
            is_smart_account_active: params.is_smart_account_active,
            circuit_breaker_controllers: params.circuit_breaker_controllers,
        }
    }
}

impl From<SmartAccountParams> for Params {
    fn from(params: SmartAccountParams) -> Self {
        Self {
            maximum_unauthenticated_gas: params.maximum_unauthenticated_gas,
            is_smart_account_active: params.is_smart_account_active,
            circuit_breaker_controllers: params.circuit_breaker_controllers,
        }
    }
}

/// Account authenticator information
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AccountAuthenticatorInfo {
    /// Unique identifier for the authenticator
    pub id: u64,
    /// Type of authenticator
    pub authenticator_type: String,
    /// Configuration data for the authenticator
    pub config: Vec<u8>,
}

impl From<AccountAuthenticator> for AccountAuthenticatorInfo {
    fn from(auth: AccountAuthenticator) -> Self {
        Self {
            id: auth.id,
            authenticator_type: auth.r#type,
            config: auth.config,
        }
    }
}

impl From<AccountAuthenticatorInfo> for AccountAuthenticator {
    fn from(info: AccountAuthenticatorInfo) -> Self {
        Self {
            id: info.id,
            r#type: info.authenticator_type,
            config: info.config,
        }
    }
}

/// Authenticator data for an account
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthenticatorDataInfo {
    /// Account address
    pub address: String,
    /// List of authenticators for the account
    pub authenticators: Vec<AccountAuthenticatorInfo>,
}

impl From<AuthenticatorData> for AuthenticatorDataInfo {
    fn from(data: AuthenticatorData) -> Self {
        Self {
            address: data.address,
            authenticators: data
                .authenticators
                .into_iter()
                .map(AccountAuthenticatorInfo::from)
                .collect(),
        }
    }
}

impl From<AuthenticatorDataInfo> for AuthenticatorData {
    fn from(info: AuthenticatorDataInfo) -> Self {
        Self {
            address: info.address,
            authenticators: info
                .authenticators
                .into_iter()
                .map(AccountAuthenticator::from)
                .collect(),
        }
    }
}

/// Message to add an authenticator
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MsgAddAccountAuthenticator {
    /// Sender address
    pub sender: String,
    /// Type of authenticator
    pub authenticator_type: String,
    /// Configuration data
    pub data: Vec<u8>,
}

impl From<MsgAddAccountAuthenticator> for MsgAddAuthenticator {
    fn from(msg: MsgAddAccountAuthenticator) -> Self {
        Self {
            sender: msg.sender,
            authenticator_type: msg.authenticator_type,
            data: msg.data,
        }
    }
}

impl TryFrom<MsgAddAuthenticator> for MsgAddAccountAuthenticator {
    type Error = Error;

    fn try_from(proto: MsgAddAuthenticator) -> Result<Self> {
        Ok(Self {
            sender: proto.sender,
            authenticator_type: proto.authenticator_type,
            data: proto.data,
        })
    }
}

/// Message to remove an authenticator
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MsgRemoveAccountAuthenticator {
    /// Sender address
    pub sender: String,
    /// ID of the authenticator to remove
    pub id: u64,
}

impl From<MsgRemoveAccountAuthenticator> for MsgRemoveAuthenticator {
    fn from(msg: MsgRemoveAccountAuthenticator) -> Self {
        Self {
            sender: msg.sender,
            id: msg.id,
        }
    }
}

impl TryFrom<MsgRemoveAuthenticator> for MsgRemoveAccountAuthenticator {
    type Error = Error;

    fn try_from(proto: MsgRemoveAuthenticator) -> Result<Self> {
        Ok(Self {
            sender: proto.sender,
            id: proto.id,
        })
    }
}

/// Message to set active state of smart account functionality
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MsgSetSmartAccountActiveState {
    /// Sender address (must be a circuit breaker controller)
    pub sender: String,
    /// Whether to activate or deactivate smart accounts
    pub active: bool,
}

impl From<MsgSetSmartAccountActiveState> for MsgSetActiveState {
    fn from(msg: MsgSetSmartAccountActiveState) -> Self {
        Self {
            sender: msg.sender,
            active: msg.active,
        }
    }
}

impl TryFrom<MsgSetActiveState> for MsgSetSmartAccountActiveState {
    type Error = Error;

    fn try_from(proto: MsgSetActiveState) -> Result<Self> {
        Ok(Self {
            sender: proto.sender,
            active: proto.active,
        })
    }
}

/// Query request for smart account module parameters
pub struct QuerySmartAccountParamsRequest {}

impl From<QuerySmartAccountParamsRequest> for QueryParamsRequest {
    fn from(_: QuerySmartAccountParamsRequest) -> Self {
        Self {}
    }
}

impl From<QueryParamsRequest> for QuerySmartAccountParamsRequest {
    fn from(_: QueryParamsRequest) -> Self {
        Self {}
    }
}

/// Query response for smart account module parameters
pub struct QuerySmartAccountParamsResponse {
    /// The module parameters
    pub params: Option<SmartAccountParams>,
}

impl From<QueryParamsResponse> for QuerySmartAccountParamsResponse {
    fn from(proto: QueryParamsResponse) -> Self {
        Self {
            params: proto.params.map(|p| p.into()),
        }
    }
}

impl From<QuerySmartAccountParamsResponse> for QueryParamsResponse {
    fn from(resp: QuerySmartAccountParamsResponse) -> Self {
        Self {
            params: resp.params.map(|p| p.into()),
        }
    }
}
