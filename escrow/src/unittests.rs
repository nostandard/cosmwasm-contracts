use cosmwasm_std::{coins, from_json, Timestamp};
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Addr,
};
use cw_utils::Expiration;

use crate::{
    contract,
    msg::{AgentResp, InstantiateMsg, QueryMsg},
};

const TEST_TOKEN: &str = "ATOM";
const EXPIRATION: Expiration = Expiration::AtHeight(1000);

fn init_msg_ctor(expiration: Option<Expiration>) -> InstantiateMsg {
    InstantiateMsg {
        recipient: "recipient".to_owned(),
        agent: "agent".to_owned(),
        expiration,
    }
}

#[test]
fn instantiate_and_query_agent() {
    let mut deps = mock_dependencies();
    let init_msg = init_msg_ctor(Some(EXPIRATION));
    let msg_info = mock_info("creator", &coins(500, TEST_TOKEN));

    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(0);
    env.block.height = 900; // this block height < `EXPIRATION`... -> success!

    let resp = contract::instantiate(deps.as_mut(), env.clone(), msg_info, init_msg).unwrap();
    assert_eq!(0, resp.messages.len());

    let agent_resp: AgentResp =
        from_json(contract::query(deps.as_ref(), env, QueryMsg::Agent {}).unwrap()).unwrap();
    assert_eq!(
        agent_resp,
        AgentResp {
            agent: Addr::unchecked("agent")
        }
    );
}

#[test]
fn instantiate_on_expiration() {
    let mut deps = mock_dependencies();
    let init_msg = init_msg_ctor(Some(EXPIRATION));
    let msg_info = mock_info("creator", &coins(500, TEST_TOKEN));

    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(0);
    env.block.height = 1005; // this block height > `EXPIRATION`... -> failure!

    assert!(contract::instantiate(deps.as_mut(), env, msg_info, init_msg).is_err());
}
