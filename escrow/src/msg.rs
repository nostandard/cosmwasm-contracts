use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};
use cw_utils::Expiration;

#[cw_serde]
pub struct InstantiateMsg {
    pub recipient: String,
    pub arbiter: String,
    pub escrow_token: String,
    pub expiration: Option<Expiration>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Withdraw { amount: Option<Vec<Coin>> },
    Refund {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ArbiterResp)]
    Arbiter {},
}

#[cw_serde]
pub struct ArbiterResp {
    pub arbiter: Addr,
}
