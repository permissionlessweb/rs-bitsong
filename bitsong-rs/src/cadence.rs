//! Cadence module support for the Bitsong blockchain.
//!
//! This module provides support for interacting with Bitsong's Cadence module,
//! which handles contract management and execution limits.

use rs_bitsong_proto::types::bitsong::cadence::v1::{
    CadenceContract, MsgRegisterCadenceContract, MsgUnjailCadenceContract,
    MsgUnregisterCadenceContract, MsgUpdateParams, Params, QueryCadenceContract,
    QueryCadenceContractResponse, QueryCadenceContracts, QueryCadenceContractsResponse,
    QueryParamsRequest, QueryParamsResponse,
};

use crate::{Error, Result};
use serde::{Deserialize, Serialize};

pub use rs_bitsong_proto::types::bitsong::cadence::v1::{
    CadenceContract as ProtoCadenceContract, Params as ProtoParams,
};

/// Cadence contract information
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CadenceContractInfo {
    /// The address of the contract
    pub contract_address: String,
    /// Whether the contract is jailed
    pub is_jailed: bool,
}

impl From<CadenceContract> for CadenceContractInfo {
    fn from(contract: CadenceContract) -> Self {
        Self {
            contract_address: contract.contract_address,
            is_jailed: contract.is_jailed,
        }
    }
}

impl From<CadenceContractInfo> for CadenceContract {
    fn from(info: CadenceContractInfo) -> Self {
        Self {
            contract_address: info.contract_address,
            is_jailed: info.is_jailed,
        }
    }
}

/// Cadence module parameters
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CadenceParams {
    /// Maximum amount of gas that can be used by a contract
    pub contract_gas_limit: u64,
}

impl From<Params> for CadenceParams {
    fn from(params: Params) -> Self {
        Self {
            contract_gas_limit: params.contract_gas_limit,
        }
    }
}

impl From<CadenceParams> for Params {
    fn from(params: CadenceParams) -> Self {
        Self {
            contract_gas_limit: params.contract_gas_limit,
        }
    }
}

/// Message to add a cadence contract
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MsgAddContract {
    /// Authority is the address that controls the module
    pub authority: String,
    /// The contract address to add
    pub contract_address: String,
}

impl From<MsgAddContract> for MsgRegisterCadenceContract {
    fn from(msg: MsgAddContract) -> Self {
        Self {
            sender_address: msg.authority,
            contract_address: msg.contract_address,
        }
    }
}

impl TryFrom<MsgRegisterCadenceContract> for MsgAddContract {
    type Error = Error;

    fn try_from(proto: MsgRegisterCadenceContract) -> Result<Self> {
        Ok(Self {
            authority: proto.sender_address,
            contract_address: proto.contract_address,
        })
    }
}

/// Message to remove a cadence contract
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MsgRemoveContract {
    /// Authority is the address that controls the module
    pub authority: String,
    /// The contract address to remove
    pub contract_address: String,
}

impl From<MsgRemoveContract> for MsgUnregisterCadenceContract {
    fn from(msg: MsgRemoveContract) -> Self {
        Self {
            sender_address: msg.authority,
            contract_address: msg.contract_address,
        }
    }
}

impl TryFrom<MsgUnregisterCadenceContract> for MsgRemoveContract {
    type Error = Error;

    fn try_from(proto: MsgUnregisterCadenceContract) -> Result<Self> {
        Ok(Self {
            authority: proto.sender_address,
            contract_address: proto.contract_address,
        })
    }
}

/// Message to unjail a cadence contract
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MsgUnjailContract {
    /// Authority is the address that controls the module
    pub authority: String,
    /// The contract address to unjail
    pub contract_address: String,
}

impl From<MsgUnjailContract> for MsgUnjailCadenceContract {
    fn from(msg: MsgUnjailContract) -> Self {
        Self {
            sender_address: msg.authority,
            contract_address: msg.contract_address,
        }
    }
}

impl TryFrom<MsgUnjailCadenceContract> for MsgUnjailContract {
    type Error = Error;

    fn try_from(proto: MsgUnjailCadenceContract) -> Result<Self> {
        Ok(Self {
            authority: proto.sender_address,
            contract_address: proto.contract_address,
        })
    }
}

/// Message to update cadence module parameters
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MsgUpdateCadenceParams {
    /// Authority is the address that controls the module
    pub authority: String,
    /// New parameters
    pub params: CadenceParams,
}

impl From<MsgUpdateCadenceParams> for MsgUpdateParams {
    fn from(msg: MsgUpdateCadenceParams) -> Self {
        Self {
            authority: msg.authority,
            params: Some(msg.params.into()),
        }
    }
}

impl TryFrom<MsgUpdateParams> for MsgUpdateCadenceParams {
    type Error = Error;

    fn try_from(proto: MsgUpdateParams) -> Result<Self> {
        let params = proto
            .params
            .ok_or_else(|| Error::Cadence("missing params in MsgUpdateParams".to_string()))?;

        Ok(Self {
            authority: proto.authority,
            params: params.into(),
        })
    }
}

/// Query request for cadence module parameters
pub struct QueryCadenceParams {}

impl From<QueryCadenceParams> for QueryParamsRequest {
    fn from(_: QueryCadenceParams) -> Self {
        Self {}
    }
}

impl From<QueryParamsRequest> for QueryCadenceParams {
    fn from(_: QueryParamsRequest) -> Self {
        Self {}
    }
}

/// Query response for cadence module parameters
pub struct QueryCadenceParamsResp {
    /// The module parameters
    pub params: Option<CadenceParams>,
}

impl From<QueryParamsResponse> for QueryCadenceParamsResp {
    fn from(proto: QueryParamsResponse) -> Self {
        Self {
            params: proto.params.map(|p| p.into()),
        }
    }
}

impl From<QueryCadenceParamsResp> for QueryParamsResponse {
    fn from(resp: QueryCadenceParamsResp) -> Self {
        Self {
            params: resp.params.map(|p| p.into()),
        }
    }
}

/// Query request for a specific cadence contract
pub struct QueryCadenceContractRequest {
    /// The address of the contract to query
    pub contract_address: String,
}

impl From<QueryCadenceContractRequest> for QueryCadenceContract {
    fn from(req: QueryCadenceContractRequest) -> Self {
        Self {
            contract_address: req.contract_address,
        }
    }
}

impl From<QueryCadenceContract> for QueryCadenceContractRequest {
    fn from(proto: QueryCadenceContract) -> Self {
        Self {
            contract_address: proto.contract_address,
        }
    }
}

/// Query response for a specific cadence contract
pub struct QueryCadenceContractResp {
    /// The cadence contract information
    pub cadence_contract: Option<CadenceContractInfo>,
}

impl From<QueryCadenceContractResponse> for QueryCadenceContractResp {
    fn from(proto: QueryCadenceContractResponse) -> Self {
        Self {
            cadence_contract: proto.cadence_contract.map(|c| c.into()),
        }
    }
}

impl From<QueryCadenceContractResp> for QueryCadenceContractResponse {
    fn from(resp: QueryCadenceContractResp) -> Self {
        Self {
            cadence_contract: resp.cadence_contract.map(|c| c.into()),
        }
    }
}

/// Query request for all cadence contracts
pub struct QueryCadenceContractsRequest {
    /// Pagination parameters
    pub pagination: Option<crate::types::cosmos::base::query::v1beta1::PageRequest>,
}

impl From<QueryCadenceContractsRequest> for QueryCadenceContracts {
    fn from(req: QueryCadenceContractsRequest) -> Self {
        Self {
            pagination: req.pagination,
        }
    }
}

impl From<QueryCadenceContracts> for QueryCadenceContractsRequest {
    fn from(proto: QueryCadenceContracts) -> Self {
        Self {
            pagination: proto.pagination,
        }
    }
}

/// Query response for all cadence contracts
pub struct QueryCadenceContractsResp {
    /// List of cadence contracts
    pub cadence_contracts: Vec<CadenceContractInfo>,
    /// Pagination information
    pub pagination: Option<crate::types::cosmos::base::query::v1beta1::PageResponse>,
}

impl From<QueryCadenceContractsResponse> for QueryCadenceContractsResp {
    fn from(proto: QueryCadenceContractsResponse) -> Self {
        Self {
            cadence_contracts: proto
                .cadence_contracts
                .into_iter()
                .map(|c| c.into())
                .collect(),
            pagination: proto.pagination,
        }
    }
}

impl From<QueryCadenceContractsResp> for QueryCadenceContractsResponse {
    fn from(resp: QueryCadenceContractsResp) -> Self {
        Self {
            cadence_contracts: resp
                .cadence_contracts
                .into_iter()
                .map(|c| c.into())
                .collect(),
            pagination: resp.pagination,
        }
    }
}
