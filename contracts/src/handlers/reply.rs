use cosmwasm_std::{DepsMut, Env, Reply};

use crate::contract::{CounterApp, CounterResult};

pub fn example_reply_handler(
    _deps: DepsMut,
    _env: Env,
    _app: CounterApp,
    _reply: Reply,
) -> CounterResult {
    // Logic to execute on example reply
    todo!()
}
