#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::state::STATE;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetOptAddr { new_opt_addr } => execute::set_opt_addr(deps, new_opt_addr),
        ExecuteMsg::SetOptUint128 { new_opt_uint128 } => {
            execute::set_opt_uint128(deps, new_opt_uint128)
        }
        ExecuteMsg::SetOptVecNumbers {
            new_opt_vec_numbers,
        } => execute::set_opt_vec_numbers(deps, new_opt_vec_numbers),
        ExecuteMsg::SetVecOptStrings {
            new_vec_opt_strings,
        } => execute::set_vec_opt_strings(deps, new_vec_opt_strings),
        ExecuteMsg::SetVecDenomUnits {
            new_vec_denom_units,
        } => execute::set_vec_denom_units(deps, new_vec_denom_units),
        ExecuteMsg::UpdateOwner(update) => execute::update_owner(deps, info, update),
    }
}

pub mod execute {
    use cosmwasm_std::{DenomUnit, Uint128};

    use crate::{state::OWNER, types::owner::OwnerUpdate};

    use super::*;

    pub fn set_opt_addr(
        deps: DepsMut,
        new_opt_addr: Option<String>,
    ) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.opt_addr = match new_opt_addr.clone() {
                Some(addr) => Some(deps.api.addr_validate(&addr)?),
                _ => None,
            };
            Ok(state)
        })?;
        Ok(Response::new()
            .add_attribute("new_opt_addr", new_opt_addr.unwrap_or("None".to_string())))
    }

    pub fn set_opt_uint128(
        deps: DepsMut,
        new_opt_uint128: Option<Uint128>,
    ) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.opt_uint128 = new_opt_uint128;
            Ok(state)
        })?;
        Ok(Response::new().add_attribute(
            "new_opt_uint128",
            match new_opt_uint128 {
                Some(uint128) => uint128.to_string(),
                _ => "None".to_string(),
            },
        ))
    }

    pub fn set_opt_vec_numbers(
        deps: DepsMut,
        new_opt_vec_numbers: Option<Vec<i32>>,
    ) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.opt_vec_numbers = new_opt_vec_numbers;
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("new_opt_vec_numbers", "new_vec_numbers"))
    }

    pub fn set_vec_opt_strings(
        deps: DepsMut,
        new_vec_opt_strings: Vec<Option<String>>,
    ) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.vec_opt_strings = new_vec_opt_strings;
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("new_vec_opt_strings", "new_vec_opt_strings"))
    }

    pub fn set_vec_denom_units(
        deps: DepsMut,
        new_vec_denom_units: Vec<DenomUnit>,
    ) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.vec_denom_units = new_vec_denom_units;
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("new_vec_denom_units", "new_vec_denom_units"))
    }

    pub fn update_owner(
        deps: DepsMut,
        info: MessageInfo,
        update: OwnerUpdate,
    ) -> Result<Response, ContractError> {
        Ok(OWNER.update(deps, info, update)?)
    }
}
