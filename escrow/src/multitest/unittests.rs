use cosmwasm_std::{coins, Timestamp};
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Addr,
};
use cw_utils::Expiration;

use crate::{
    contract,
    msg::InstantiateMsg,
    state::{State, STATE},
};

const TEST_TOKEN: &str = "ATOM";

#[test]
fn init_and_query_agent() {
    let mut deps = mock_dependencies();

    let init_msg = InstantiateMsg {
        recipient: "recipient".to_owned(),
        agent: "agent".to_owned(),
        expiration: Some(Expiration::AtHeight(1000)),
    };

    let mut env = mock_env();
    env.block.height = 900;
    env.block.time = Timestamp::from_seconds(0);
    let msg_info = mock_info("creator", &coins(500, TEST_TOKEN));

    let resp = contract::instantiate(deps.as_mut(), env, msg_info, init_msg).unwrap();
    assert_eq!(0, resp.messages.len());

    let state = STATE.load(&deps.storage).unwrap();
    assert_eq!(
        state,
        State {
            creator: Addr::unchecked("creator"),
            recipient: Addr::unchecked("recipient"),
            agent: Addr::unchecked("agent"),
            expiration: Some(Expiration::AtHeight(1000))
        }
    );
}
