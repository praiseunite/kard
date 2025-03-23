use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("GiftNotFound")]
    GiftNotFound {},

    #[error("GiftAlreadyClaimed")]
    GiftAlreadyClaimed {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}

#[derive(Debug)]
pub enum AuthError {
    EmailAlreadyLinked {},
    Std(cosmwasm_std::StdError),
}

impl From<cosmwasm_std::StdError> for AuthError {
    fn from(err: cosmwasm_std::StdError) -> Self {
        AuthError::Std(err)
    }
}

#[derive(Debug)]
#[allow(unused)]
pub enum DepositError {
    WalletNotFound {},
    Std(cosmwasm_std::StdError),
}

impl From<cosmwasm_std::StdError> for DepositError {
    fn from(err: cosmwasm_std::StdError) -> Self {
        DepositError::Std(err)
    }
}
