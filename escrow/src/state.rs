use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use cw_utils::Expiration;

#[cw_serde]
pub struct State {
    pub creator: Addr,
    pub recipient: Addr,
    pub agent: Addr,
    pub expiration: Option<Expiration>,
}

pub const STATE: Item<State> = Item::new("state");
