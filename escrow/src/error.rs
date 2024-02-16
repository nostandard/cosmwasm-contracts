use cosmwasm_std::{Addr, StdError};
use cw_utils::{Expiration, PaymentError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),

    #[error("{sender} is not contract admin")]
    Unauthorized { sender: Addr },

    #[error("Escrow has expired (expiration: {expiration:?})")]
    Expired { expiration: Expiration },

    #[error("Escrow has not yet expired")]
    NotExpired {},

    #[error("Specified amount is in excess of balance")]
    InsufficientFunds,
}
