use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, DenomUnit};

#[cw_serde]
pub enum Object {
    EnumOption1,
    EnumOption2,
    Number { value: i32 },
    OptNumber { value: Option<i32> },
    String { value: String },
    OptString { value: Option<String> },
    DenomUnit { value: DenomUnit },
    OptVecDenomUnit { value: Option<Vec<DenomUnit>> },
    VecU8 { value: Binary },
}
