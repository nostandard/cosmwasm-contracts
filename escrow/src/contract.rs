use crate::{
    error::ContractError,
    msg::{ArbiterResp, ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{Escrow, ESCROW, ESCROW_TOKEN},
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
    let escrow = Escrow {
        creator: info.sender,
        arbiter: deps.api.addr_validate(&msg.arbiter)?,
        recipient: deps.api.addr_validate(&msg.recipient)?,
        expiration: msg.expiration,
        balance: deps.querier.query_all_balances(env.contract.address)?,
    };

    if info.funds.is_empty() {
        return Err(ContractError::NoFunds);
    }

    if let Some(expiration) = msg.expiration {
        if expiration.is_expired(&env.block) {
            return Err(ContractError::Expired { expiration });
        }
    }
    ESCROW.save(deps.storage, &escrow)?;
    ESCROW_TOKEN.save(deps.storage, &msg.escrow_token)?;

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
        Arbiter {} => to_json_binary(&query::query_arbiter(deps)?),
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
        let escrow = ESCROW.load(deps.storage)?;
        let token = ESCROW_TOKEN.load(deps.storage)?;

        // Only agent is authorized to make a withdrawal
        if info.sender != escrow.arbiter {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }

        if let Some(expiration) = escrow.expiration {
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
                to_address: escrow.recipient.to_string(),
                amount,
            })
            .add_attribute("action", "withdraw")
            .add_attribute("recipient", escrow.recipient.as_str())
            .add_attribute("executed_by", info.sender.to_string())
            .add_attribute("token", token);

        Ok(resp)
    }

    pub fn refund(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
        let escrow = ESCROW.load(deps.storage)?;
        let token = ESCROW_TOKEN.load(deps.storage)?;

        // Only agent is authorized to issue a refund
        if info.sender != escrow.arbiter {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }

        if let Some(expiration) = escrow.expiration {
            if !expiration.is_expired(&env.block) {
                return Err(ContractError::NotExpired {});
            }
        } else {
            return Err(ContractError::NotExpired {});
        }

        let deposit = deps.querier.query_all_balances(&env.contract.address)?;

        let resp = Response::new()
            .add_message(BankMsg::Send {
                to_address: escrow.creator.to_string(),
                amount: deposit,
            })
            .add_attribute("action", "refund")
            .add_attribute("recipient", escrow.creator.as_str())
            .add_attribute("executed_by", info.sender.to_string())
            .add_attribute("token", token);

        Ok(resp)
    }
}

mod query {
    use super::*;

    pub fn query_arbiter(deps: Deps) -> StdResult<ArbiterResp> {
        let escrow = ESCROW.load(deps.storage)?;
        let arbiter_addr = escrow.arbiter;
        Ok(ArbiterResp {
            arbiter: arbiter_addr,
        })
    }
}
