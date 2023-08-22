use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{DenomUnit, Uint128};

use crate::{
    state::State,
    types::{
        object::Object,
        owner::{OwnerResponse, OwnerUpdate},
    },
};

#[cw_serde]
pub struct InstantiateMsg {
    /// opt_uint128 - Option<Uint128>
    pub opt_uint128: Option<Uint128>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Set new Option<Addr>.
    SetOptAddr {
        /// new_opt_addr - Option<String>
        new_opt_addr: Option<String>,
    },
    /// Set new Option<Uint128>.
    SetOptUint128 {
        /// new_opt_uint128 - Option<Uint128>
        new_opt_uint128: Option<Uint128>,
    },
    /// Set new Option<Vec<i32>>.
    SetOptVecNumbers {
        /// new_opt_vec_numbers - Option<Vec<i32>>
        new_opt_vec_numbers: Option<Vec<i32>>,
    },
    /// Set new Vec<Option<String>>.
    SetVecOptStrings {
        /// new_vec_opt_strings - Vec<Option<String>>
        new_vec_opt_strings: Vec<Option<String>>,
    },
    /// Set new Vec<DenomUnit>>.
    SetVecDenomUnits {
        /// new_vec_denom_units - Vec<DenomUnit>
        /// struct DenomUnit {
        ///     denom: String,
        ///     exponent: u32,
        ///     aliases: Vec<String>,
        /// }
        new_vec_denom_units: Vec<DenomUnit>,
    },
    /// Update OWNER item.
    UpdateOwner(OwnerUpdate),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// GetOwner returns the current OWNER state saved in the contract.
    #[returns(GetOwnerResponse)]
    GetOwner {},
    /// GetState returns the current state saved in the contract.
    #[returns(GetStateResponse)]
    GetState {},
    /// GetObject returns the input directly
    #[returns(GetObjectResponse)]
    GetObject { object: Object },
}

// We define a custom struct for each query response
#[cw_serde]
/// Response from get_owner
pub struct GetOwnerResponse {
    /// current OWNER state
    pub owner_response: OwnerResponse,
}

#[cw_serde]
/// Response from get_state
pub struct GetStateResponse {
    /// current contract state
    pub state: State,
}

#[cw_serde]
/// Response from get_object.
pub struct GetObjectResponse {
    /// object result
    pub object: Object,
}

#[cw_serde]
pub struct MigrateMsg {
    /// object (for testing, not used)
    pub object: Object,
}
