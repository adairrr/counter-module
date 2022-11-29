use abstract_app::export_endpoints;
use abstract_app::AppContract;
use abstract_sdk::os::EXCHANGE;
use cosmwasm_std::Response;

use crate::error::CounterError;
use crate::handlers::{self};
use crate::msg::{
    TemplateExecuteMsg, TemplateInstantiateMsg, TemplateMigrateMsg, TemplateQueryMsg,
};

// As an app writer, the only changes necessary to this file are with the handlers and API dependencies on the `counter_app` const.
pub type CounterApp = AppContract<
    CounterError,
    TemplateExecuteMsg,
    TemplateInstantiateMsg,
    TemplateQueryMsg,
    TemplateMigrateMsg,
>;

pub type CounterResult = Result<Response, CounterError>;

/// The namespace for the app, like "abstract" -> "abstract:template"
pub const MODULE_NAMESPACE: &str = "testing";
/// The name of the app, excluding the namespace
pub const MODULE_NAME: &str = "counter_app";
/// The initial version of the app, which will use the package version if not altered
const MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Expected replies
pub const EXAMPLE_REPLY_ID: u64 = 1;

/// Used as the foundation for building your app.
/// All entrypoints are executed through this const (`instantiate`, `query`, `execute`, `migrate`)
/// The `dependencies` are Abstract API dependencies in the format: Vec(`namespace:contract_name`)
const APP: CounterApp = CounterApp::new(MODULE_NAME, MODULE_VERSION)
    .with_instantiate(handlers::instantiate_handler)
    .with_query(handlers::query_handler)
    .with_execute(handlers::execute_handler)
    .with_migrate(handlers::migrate_handler)
    .with_dependencies(&[EXCHANGE]);

// don't export endpoints when imported as library
#[cfg(not(feature = "library"))]
// Export the endpoints for this contract
export_endpoints!(APP, CounterApp);
