#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult};

use crate::msg::QueryMsg;
use crate::state::{OWNER, STATE};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&query::owner(deps)?),
        QueryMsg::GetState {} => to_binary(&query::state(deps)?),
        QueryMsg::GetObject { object } => to_binary(&query::object(object)?),
        QueryMsg::GetDenomUnit(denom_unit) => to_binary(&query::denom_unit(denom_unit)?),
        QueryMsg::GetString(string) => to_binary(&query::string(string)?),
        QueryMsg::GetNumber(number) => to_binary(&query::number(number)?),
        QueryMsg::GetBool(bool) => to_binary(&query::bool(bool)?),
        QueryMsg::QueryMessage1 => to_binary(&query::query_message_1()?),
        QueryMsg::QueryMessage2 => to_binary(&query::query_message_2()?),
    }
}

pub mod query {
    use cosmwasm_std::DenomUnit;

    use crate::{
        msg::{
            GetBoolResponse, GetDenomUnitResponse, GetNumberResponse, GetObjectResponse,
            GetOwnerResponse, GetStateResponse, GetStringResponse, QueryMessage1Response,
            QueryMessage2Response,
        },
        types::object::Object,
    };

    use super::*;

    pub fn owner(deps: Deps) -> StdResult<GetOwnerResponse> {
        Ok(GetOwnerResponse {
            owner_response: OWNER.query(deps.storage)?,
        })
    }

    pub fn state(deps: Deps) -> StdResult<GetStateResponse> {
        Ok(GetStateResponse {
            state: STATE.load(deps.storage)?,
        })
    }

    pub fn object(object: Object) -> StdResult<GetObjectResponse> {
        Ok(GetObjectResponse { object })
    }

    pub fn denom_unit(denom_unit: DenomUnit) -> StdResult<GetDenomUnitResponse> {
        Ok(GetDenomUnitResponse(denom_unit))
    }

    pub fn string(string: String) -> StdResult<GetStringResponse> {
        Ok(GetStringResponse(string))
    }

    pub fn number(number: u32) -> StdResult<GetNumberResponse> {
        Ok(GetNumberResponse(number))
    }

    pub fn bool(bool: bool) -> StdResult<GetBoolResponse> {
        Ok(GetBoolResponse(bool))
    }

    pub fn query_message_1() -> StdResult<QueryMessage1Response> {
        Ok(QueryMessage1Response)
    }

    pub fn query_message_2() -> StdResult<QueryMessage2Response> {
        Ok(QueryMessage2Response {})
    }
}
