// use secp256k1::{Message, PublicKey, Secp256k1, ecdsa::Signature};
use sha2::{Digest, Sha256};
use std::str::FromStr;
use cosmwasm_std::{Addr, StdError, StdResult, Storage, Uint128};

// Use the existing storage mappings from the main contract
use crate::state::{Wallet, EMAILS, WALLETS};  

/// Saves a wallet address and email to storage
pub fn save_wallet(
    storage: &mut dyn Storage,
    wallet_address: Addr,
    email: String,
) -> StdResult<()> {
    // Check if the email is already linked to another wallet
    if EMAILS.may_load(storage, email.clone())?.is_some() {
        return Err(StdError::generic_err("Email is already linked to a wallet"));
    }

    // Initialize wallet with zero balance
    let wallet = Wallet::new(
        wallet_address.clone(),
        email.clone(),
        Uint128::zero(),
    );

    // Save the wallet and email mapping
    WALLETS.save(storage, wallet_address.clone(), &wallet)?;
    EMAILS.save(storage, email, &wallet_address)?;

    Ok(())
}

// pub fn verify_signature(
//     message_hash: &str,
//     signature: &str,
//     sender: &str,  // Changed Addr to &str
// ) -> StdResult<bool> {
//     let secp = Secp256k1::new();

//     // Hash message
//     let hashed_msg = Sha256::digest(message_hash.as_bytes());
//     let msg = Message::from_slice(&hashed_msg).map_err(|_| cosmwasm_std::StdError::generic_err("Invalid hash"))?;

//     let sig = Signature::from_str(signature).map_err(|_| cosmwasm_std::StdError::generic_err("Invalid signature"))?;
    
//     let sender_pubkey = PublicKey::from_str(sender).map_err(|_| cosmwasm_std::StdError::generic_err("Invalid sender public key"))?;
    
//     Ok(secp.verify_ecdsa(&msg, &sig, &sender_pubkey).is_ok())
// }
