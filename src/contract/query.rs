#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult};

use crate::msg::{GetStateResponse, QueryMsg};
use crate::state::{OWNER, STATE};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&query::owner(deps)?),
        QueryMsg::GetState {} => to_binary(&query::state(deps)?),
        QueryMsg::GetObject { object } => to_binary(&query::object(object)?),
    }
}

pub mod query {
    use crate::{
        msg::{GetObjectResponse, GetOwnerResponse},
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
}
