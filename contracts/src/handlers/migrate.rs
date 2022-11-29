use cosmwasm_std::{DepsMut, Env, Response};

use crate::contract::{CounterApp, CounterResult};

use crate::msg::TemplateMigrateMsg;

/// Unused for now but provided here as an example
/// Contract version is migrated automatically
pub fn migrate_handler(
    _deps: DepsMut,
    _env: Env,
    _app: CounterApp,
    _msg: TemplateMigrateMsg,
) -> CounterResult {
    Ok(Response::default())
}
