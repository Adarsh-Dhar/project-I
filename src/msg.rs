use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub token_a: String,
    pub token_b: String,
    pub initial_swap_rate: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Liquidity Provider functions
    DepositTokenA { amount: Uint128 },
    WithdrawTokenA { amount: Uint128 },
    WithdrawTokenB { amount: Uint128 },
    UpdateSwapRate { new_rate: Uint128 },
    AddToWhitelist { address: String },
    RemoveFromWhitelist { address: String },
    SetSwapLimits { 
        address: String,
        max_per_swap: Uint128,
        daily_limit: Uint128 
    },
    TogglePause {},
    
    // Party B functions
    ExecuteSwap { amount: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetState {},
    IsWhitelisted { address: String },
    GetSwapLimits { address: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub liquidity_provider: String,
    pub token_a: String,
    pub token_b: String,
    pub swap_rate: Uint128,
    pub total_token_a: Uint128,
    pub total_token_b: Uint128,
    pub paused: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WhitelistResponse {
    pub is_whitelisted: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwapLimitResponse {
    pub max_per_swap: Uint128,
    pub daily_limit: Uint128,
    pub today_volume: Uint128,
}
