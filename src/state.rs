use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, DenomUnit, Uint128};
use cw_storage_plus::Item;

use crate::types::owner::Owner;

// version info for migration info
pub const CONTRACT_NAME: &str = "crates.io:json-template";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub opt_addr: Option<Addr>,
    pub opt_uint128: Option<Uint128>,
    pub opt_vec_numbers: Option<Vec<i32>>,
    pub vec_opt_strings: Vec<Option<String>>,
    pub vec_denom_units: Vec<DenomUnit>,
}

pub const STATE: Item<State> = Item::new("state");

/// The owner of the contract
pub const OWNER: Owner = Owner::new("owner");
