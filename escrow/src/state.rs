use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;
use cw_utils::Expiration;

#[cw_serde]
pub struct Escrow {
    pub creator: Addr,
    pub arbiter: Addr,
    pub recipient: Addr,
    pub expiration: Option<Expiration>,
    pub balance: Vec<Coin>,
}

pub const ESCROW: Item<Escrow> = Item::new("escrow");
pub const ESCROW_TOKEN: Item<String> = Item::new("escrow_token");
