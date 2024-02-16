use crate::{
    error::ContractError,
    msg::{AgentResp, ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{State, STATE},
};
use cosmwasm_std::{
    to_json_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

/// Top-level handler responsible for dispatching the contract instantiation message.
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        creator: info.sender,
        recipient: deps.api.addr_validate(&msg.recipient)?,
        agent: deps.api.addr_validate(&msg.agent)?,
        expiration: msg.expiration,
    };

    if let Some(expiration) = msg.expiration {
        if expiration.is_expired(&env.block) {
            return Err(ContractError::Expired { expiration });
        }
    }
    STATE.save(deps.storage, &state)?;

    Ok(Response::new())
}

/// Top-level handler responsible for dispatching execution messages.
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Withdraw { amount } => exec::withdraw(deps, env, info, amount),
        Refund {} => exec::refund(deps, env, info),
    }
}

/// Top-level query handler responsible for dispatching query messages.
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Agent {} => to_json_binary(&query::query_agent(deps)?),
    }
}

mod exec {
    use super::*;

    pub fn withdraw(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        amount: Option<Vec<Coin>>,
    ) -> Result<Response, ContractError> {
        let state = STATE.load(deps.storage)?;
        if info.sender != state.agent {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }

        if let Some(expiration) = state.expiration {
            if expiration.is_expired(&env.block) {
                return Err(ContractError::Expired { expiration });
            }
        }

        let amount = if let Some(amt) = amount {
            amt
        } else {
            deps.querier.query_all_balances(&env.contract.address)?
        };

        let resp = Response::new()
            .add_message(BankMsg::Send {
                to_address: state.recipient.to_string(),
                amount,
            })
            .add_attribute("action", "withdraw")
            .add_attribute("recipient", state.recipient.as_str());

        Ok(resp)
    }

    pub fn refund(deps: DepsMut, env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
        let state = STATE.load(deps.storage)?;

        if let Some(expiration) = state.expiration {
            if !expiration.is_expired(&env.block) {
                return Err(ContractError::NotExpired {});
            }
        } else {
            return Err(ContractError::NotExpired {});
        }

        let deposit = deps.querier.query_all_balances(&env.contract.address)?;

        let resp = Response::new()
            .add_message(BankMsg::Send {
                to_address: state.creator.to_string(),
                amount: deposit,
            })
            .add_attribute("action", "refund")
            .add_attribute("recipient", state.creator.as_str());

        Ok(resp)
    }
}

mod query {
    use super::*;

    pub fn query_agent(deps: Deps) -> StdResult<AgentResp> {
        let state = STATE.load(deps.storage)?;
        let agent_addr = state.agent;
        Ok(AgentResp { agent: agent_addr })
    }
}
