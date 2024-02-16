use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};
use cw_utils::Expiration;

#[cw_serde]
pub struct InstantiateMsg {
    pub recipient: String,
    pub agent: String,
    pub escrow_token: String,
    pub expiration: Option<Expiration>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Deposit { amount: Vec<Coin> },
    Withdraw { amount: Option<Vec<Coin>> },
    Refund {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AgentResp)]
    Agent {},
}

#[cw_serde]
pub struct AgentResp {
    pub agent: Addr,
}
