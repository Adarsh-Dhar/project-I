use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub liquidity_provider: Addr,
    pub token_a: Addr,
    pub token_b: Addr,
    pub swap_rate: Uint128,  // Rate is stored as token_b/token_a * 1e6
    pub total_token_a: Uint128,
    pub total_token_b: Uint128,
    pub paused: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwapLimit {
    pub max_per_swap: Uint128,
    pub daily_limit: Uint128,
    pub last_swap_time: u64,
    pub today_volume: Uint128,
}

pub const STATE: Item<State> = Item::new("state");
pub const WHITELIST: Map<&Addr, bool> = Map::new("whitelist");
pub const SWAP_LIMITS: Map<&Addr, SwapLimit> = Map::new("swap_limits");
