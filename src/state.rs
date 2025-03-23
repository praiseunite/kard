use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub(crate) enum GiftStatus {
    Unclaimed,
    Claimed,
    Expired,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub(crate) enum GiftStage {
    Sent,
    Received,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub(crate) struct GiftCard {
    name: String,
    sender: Addr,
    amount: u128,
    expiry: u64,
    hash: String,
    status: GiftStatus,
}

impl GiftCard {
    pub(crate) fn new(name: String, sender: Addr, amount: u128, expiry: u64) -> Self {
        let gift_data = format!("{}:{}:{}", sender, amount, expiry);
        let hash = hex::encode(Sha256::digest(gift_data.as_bytes()));

        Self {
            name,
            sender,
            amount,
            expiry,
            hash,
            status: GiftStatus::Unclaimed,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn sender(&self) -> &Addr {
        &self.sender
    }

    pub(crate) fn amount(&self) -> u128 {
        self.amount
    }

    pub(crate) fn expiry(&self) -> u64 {
        self.expiry
    }

    pub(crate) fn hash(&self) -> &str {
        &self.hash
    }

    pub(crate) fn status(&self) -> &GiftStatus {
        &self.status
    }

    pub(crate) fn mark_claimed(&mut self) {
        self.status = GiftStatus::Claimed;
    }

    pub(crate) fn mark_expired(&mut self) {
        self.status = GiftStatus::Expired;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub(crate) struct RedeemedGift {
    gift_card: GiftCard,
    stage: GiftStage,
}

impl RedeemedGift {
    pub(crate) fn new(gift_card: GiftCard, stage: GiftStage) -> Self {
        Self { gift_card, stage }
    }

    pub(crate) fn gift_card(&self) -> &GiftCard {
        &self.gift_card
    }

    pub(crate) fn stage(&self) -> &GiftStage {
        &self.stage
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub(crate) struct Wallet {
    address: Addr,
    email: String,
    balance: Uint128,
}

impl Wallet {
    pub(crate) fn new(address: Addr, email: String, balance: Uint128) -> Self {
        Self {
            address,
            email,
            balance,
        }
    }

    pub(crate) fn address(&self) -> &Addr {
        &self.address
    }

    pub(crate) fn email(&self) -> &String {
        &self.email
    }

    pub(crate) fn balance(&self) -> Uint128 {
        self.balance
    }

    pub(crate) fn credit(&mut self, amount: Uint128) {
        self.balance += amount;
    }

    pub(crate) fn debit(&mut self, amount: Uint128) {
        self.balance -= amount;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) struct WalletVerification {
    message: String,
}

impl WalletVerification {
    pub(crate) fn new(message: String) -> Self {
        Self { message }
    }

    // Add this getter method
    pub(crate) fn message(&self) -> &str {
        &self.message
    }
}

pub(crate) const WALLETS: Map<Addr, Wallet> = Map::new("wallets");
pub(crate) const EMAILS: Map<String, Addr> = Map::new("emails");
pub(crate) const UNCLAIMED_GIFTS: Map<String, GiftCard> = Map::new("unclaimed_gifts");
pub(crate) const REDEEMED_GIFTS: Map<(Addr, String), RedeemedGift> = Map::new("redeemed_gifts");
pub(crate) const EXPIRED_GIFTS: Map<String, GiftCard> = Map::new("expired_gifts");
