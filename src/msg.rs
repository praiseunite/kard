use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    GiveGift {
        recipient: String,  // Address of the recipient
        amount: u128,
        expiry: u64,
    },
    RedeemGift {
        sender: String,  // Changed from Addr to String
        amount: u128,
        expiry: u64,
        signature: String,
    },
    CreateWallet { 
        email: String, 
        wallet_address: Addr 
    },
    DepositFunds { 
        amount: Uint128 
    },
}
