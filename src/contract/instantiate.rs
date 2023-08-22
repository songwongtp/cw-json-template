#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use crate::state::{State, CONTRACT_NAME, CONTRACT_VERSION, OWNER, STATE};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    OWNER.initialize(
        deps.storage,
        deps.api,
        info.sender.to_string(),
        Some("initializer".to_string()),
    )?;

    let state = State {
        opt_addr: Some(info.sender.clone()),
        opt_uint128: None,
        opt_vec_numbers: None,
        vec_opt_strings: [].to_vec(),
        vec_denom_units: [].to_vec(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("opt_addr", info.sender)
        .add_attribute(
            "opt_uint128",
            match msg.opt_uint128 {
                Some(uint128) => uint128.to_string(),
                _ => "None".to_string(),
            },
        ))
}
