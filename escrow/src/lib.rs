use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

mod contract;

#[entry_point]
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: Empty) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: Empty) -> StdResult<Binary> {
    contract::query(deps, env, msg)
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: Empty) -> StdResult<Response> {
    contract::execute(deps, env, info, msg)
}
