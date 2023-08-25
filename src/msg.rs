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
    /// Fixed string
    ExecuteMessage,
    /// Empty message do nothing
    Empty {},
    /// String message do nothing
    String(String),
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
    /// GetDenomUnit message
    #[returns(GetDenomUnitResponse)]
    GetDenomUnit(DenomUnit),
    /// String message
    #[returns(GetStringResponse)]
    GetString(String),
    /// Number message
    #[returns(GetNumberResponse)]
    GetNumber(u32),
    /// Bool message
    #[returns(GetBoolResponse)]
    GetBool(bool),
    /// Fix String Option 1
    #[returns(QueryMessage1Response)]
    QueryMessage1,
    /// Fix String Option 2
    #[returns(QueryMessage2Response)]
    QueryMessage2,
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
/// Response from query_message_1.
pub struct QueryMessage1Response;

#[cw_serde]
/// Response from query_message_2
pub struct QueryMessage2Response {}

#[cw_serde]
/// Response from get_denom_unit.
pub struct GetDenomUnitResponse(pub DenomUnit);

#[cw_serde]
/// Response from get_string.
pub struct GetStringResponse(pub String);

#[cw_serde]
/// Response from get_number.
pub struct GetNumberResponse(pub u32);

#[cw_serde]
/// Response from get_bool.
pub struct GetBoolResponse(pub bool);

#[cw_serde]
pub struct MigrateMsg {
    /// object (for testing, not used)
    pub object: Object,
}
