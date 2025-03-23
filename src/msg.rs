use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    GiveGift {
        amount: u128,
        expiry: u64,
    },
    RedeemGift {
        sender: String, // Changed from Addr to String
        amount: u128,
        expiry: u64,
        signature: String,
    },
    CreateWallet {
        email: String,
        wallet_address: Addr,
    },
    DepositFunds {
        amount: Uint128,
    },
}

#[cw_serde]
pub enum QueryMsg {
    GetWalletByEmail { email: String },

    ValidateWallet { address: Addr },
    // You can add more query types here as needed
}
