use cosmwasm_std::{
    entry_point, to_json_binary, Addr, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128
};
use sha2::{Digest, Sha256};
use rand::distr::Alphanumeric;
use rand::Rng;
use crate::error::{AuthError, DepositError};
use crate::msg::ExecuteMsg;
use crate::state::{GiftCard, GiftStage, GiftStatus, RedeemedGift, Wallet, WalletVerification, EMAILS, REDEEMED_GIFTS, UNCLAIMED_GIFTS, WALLETS};
use crate::helpers::verify_signature;
// use cosmwasm_crypto::secp256k1_verify;


// #[entry_point]
// pub fn execute(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     msg: ExecuteMsg,
// ) -> StdResult<Response> {
//     match msg {
//         ExecuteMsg::RedeemGift {
//             sender,
//             amount,
//             expiry,
//             signature,
//         } => redeem_gift(deps, env, info, sender, amount, expiry, signature),
//         ExecuteMsg::GiveGift { /* fields */ } => {
//             // Implement the logic for GiveGift
//             unimplemented!()

//         }
//         ExecuteMsg::CreateWallet { /* fields */ } => {
//             // Implement the logic for CreateWallet
//             unimplemented!()
//         }
//         ExecuteMsg::DepositFunds { /* fields */ } => {
//             // Implement the logic for DepositFunds
//             unimplemented!()
//         }
//     }
// }

//=======================
// Function to generate a 36-character secure Gift ID
pub fn generate_gift_id(_sender: &str, signature: &str) -> String {
    let hash = Sha256::digest(signature.as_bytes());
    let sig_part = hex::encode(hash)[14..20].to_string(); // Take characters 14-20
    let random_part: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(30) // Ensure total 36 characters
        .map(char::from)
        .collect();
    format!("{}{}", random_part, sig_part)
}
//=======================

/// Authenticate user after Abstraxion OTP verification
#[entry_point]
pub fn authenticate_user(
    deps: DepsMut, 
    email: String, 
    wallet_address: Addr
) -> Result<Response, AuthError> {
    // Check if the wallet already exists
    if let Some(existing_wallet) = WALLETS.may_load(deps.storage, wallet_address.clone())? {
        return Ok(Response::new()
            .add_attribute("action", "authenticate_user")
            .add_attribute("status", "wallet_found")
            .add_attribute("wallet_address", existing_wallet.address().to_string())
            .add_attribute("email", existing_wallet.email().clone())
            .add_attribute("balance", existing_wallet.balance().to_string()));
    }

    // Check if the email is already linked to another wallet
    if EMAILS.may_load(deps.storage, email.clone())?.is_some() {
        return Err(AuthError::EmailAlreadyLinked {});
    }

    // If neither exists, create a new wallet
    let new_wallet = Wallet::new(
        wallet_address.clone(),
        email.clone(),
        Uint128::zero(),
    );

    // Save wallet & email mapping
    WALLETS.save(deps.storage, wallet_address.clone(), &new_wallet)?;
    EMAILS.save(deps.storage, email.clone(), &wallet_address)?;

    Ok(Response::new()
        .add_attribute("action", "authenticate_user")
        .add_attribute("status", "wallet_created")
        .add_attribute("wallet_address", wallet_address.to_string())
        .add_attribute("email", email))
}

pub fn get_wallet_by_email(deps: Deps, email: String) -> StdResult<Addr> {
    EMAILS.load(deps.storage, email)
}

/// Function to validate if a wallet exists
fn validate_wallet(deps: Deps, wallet_address: &Addr) -> StdResult<bool> {
    match WALLETS.may_load(deps.storage, wallet_address.clone())? {
        Some(_) => Ok(true), 
        None => Err(StdError::not_found("Wallet address not found")),
    }
}

// CREATE GIFT CARD
pub fn create_gift_card(
    deps: DepsMut,
    env: Env,
    name: String,
    sender: Addr,
    amount: u128,
    duration_days: u64,
    sender_signature: &str, // Signature for validation
) -> StdResult<Response> {
    if duration_days < 1 || duration_days > 21 {
        return Err(StdError::generic_err("Gift card duration must be between 1 and 21 days"));
    }

    let mut sender_wallet = WALLETS.load(deps.storage, sender.clone())?;
    if sender_wallet.balance() < Uint128::new(amount) {
        return Err(StdError::generic_err("Insufficient balance"));
    }

    // Lock the funds
    sender_wallet.balance() -= Uint128::new(amount);
    WALLETS.save(deps.storage, sender.clone(), &sender_wallet)?;

    let expiry_time = env.block.time.seconds() + (duration_days * 86400);
    let gift_id = generate_gift_id(&sender.to_string(), sender_signature);
    let gift_data = format!("{}:{}:{}", sender, amount, expiry_time);
    let gift_card = GiftCard::new(name, sender.clone(), amount, expiry_time);

    UNCLAIMED_GIFTS.save(deps.storage, gift_id.clone(), &gift_card)?;

    Ok(Response::new()
        .add_attribute("action", "create_gift_card")
        .add_attribute("name",gift_card.name().clone())
        .add_attribute("gift_id", gift_id)
        .add_attribute("expiry", expiry_time.to_string()))
}

// REDEEM GIFT CARD
pub fn redeem_gift_card(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    gift_id: &str,
    recipient: Addr,
    sender_signature: &str,
) -> StdResult<Response> {
    let gift_card = UNCLAIMED_GIFTS.load(deps.storage, gift_id)?;

    // Expiry Check
    if env.block.time.seconds() > gift_card.expiry() {
        return Err(StdError::generic_err("Gift card expired"));
    }

    // Verify Sender's Signature
    let message = format!("{}:{}:{}", gift_card.sender(), gift_card.amount(), gift_card.expiry());
    let computed_hash = hex::encode(Sha256::digest(message.as_bytes()));
    if computed_hash != gift_card.hash() {
        return Err(StdError::generic_err("Invalid gift card"));
    }

    // Transfer funds to recipient
    let transfer_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: recipient.to_string(),
        amount: vec![cosmwasm_std::Coin {
            denom: "uxion".to_string(),
            amount: Uint128::new(gift_card.amount()),
        }],
    });

    // Mark as Claimed

    // Clone necessary fields before removing from storage
    let sender = gift_card.sender().clone();
    let amount = gift_card.amount();
    let expiry = gift_card.expiry();
    let hash = gift_card.hash().clone();

    UNCLAIMED_GIFTS.remove(deps.storage, gift_id.to_string());
    let redeemed_gift = GiftCard {
        sender: gift_card.sender.clone(),
        amount: gift_card.amount,
        expiry: gift_card.expiry,
        hash: gift_card.hash.clone(),
        status: GiftStatus::Claimed,
    };
    
    REDEEMED_GIFTS.save(deps.storage, gift_id, &redeemed_gift)?;

    Ok(Response::new()
        .add_message(transfer_msg)
        .add_attribute("action", "redeem_gift_card")
        .add_attribute("recipient", recipient.to_string())
        .add_attribute("amount", gift_card.amount.to_string()))
}

/// Check if a wallet exists and return "Verified Wallet Address"
pub fn validate_receiver_wallet(deps: Deps, wallet_address: Addr) -> StdResult<Binary> {
    let wallet_exists = WALLETS.may_load(deps.storage, wallet_address.clone())?
        .is_some();

    if !wallet_exists {
        return Err(StdError::not_found("Wallet"));
    }

    let response = WalletVerification {
        message: "Verified Wallet Address".to_string(),
    };

    to_json_binary(&response)
}




#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, Addr, Uint128, CosmosMsg, BankMsg};

    #[test]
    fn test_redeem_gift() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let sender = "alice".to_string();
        let redeemer = Addr::unchecked("bob");
        let amount = 100_000u128;
        let expiry = env.block.time.seconds() + 3600; // 1 hour expiry

        // Simulate signing the message
        let message_hash = format!("{}:{}:{}", sender, amount, expiry);
        let signature = "valid_signature"; // Mock signature

        // Bob redeems the card
        let info = mock_info(&redeemer.to_string(), &[]);
        let res = redeem_gift(
            deps.as_mut(),
            env.clone(),
            info,
            sender.clone(),
            amount,
            expiry,
            signature.to_string(),
        );
        assert!(res.is_ok());

        // Ensure Bob got the funds
        let bank_msg = res.unwrap().messages[0].msg.clone();
        match bank_msg {
            CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
                assert_eq!(to_address, redeemer.to_string());
                assert_eq!(amount[0].amount, Uint128::new(100_000));
            }
            _ => panic!("Expected Bank Send Message"),
        }
    }
}
