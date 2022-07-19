use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub_admin_address: Addr, //juno1xyz
}
// config.admin.address
pub const CONFIG: Item<Config> = Item::new(storage_key: "config");

// state is stored on chain
