use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

/// Top-level handler responsible for dispatching the contract instantiation message.
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        sender: info.sender,
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
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}

/// Top-level query handler responsible for dispatching query messages.
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    todo!()
}
