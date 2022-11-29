use abstract_os::app::{BaseExecuteMsg, BaseQueryMsg};
use abstract_os::{app, base, extension};
use boot_core::{BootEnvironment, BootError, Contract, IndexResponse, TxResponse};
use boot_core::prelude::{boot_contract, BootExecute, BootQuery};
use cosmwasm_std::Coin;
use serde::de::DeserializeOwned;
use serde::Serialize;

use counter_app::contract::MODULE_NAME;
use counter_app::msg::{
    TemplateExecuteMsg, TemplateInstantiateMsg, TemplateMigrateMsg, TemplateQueryMsg,
};

type AppInstantiateMsg = app::InstantiateMsg<TemplateInstantiateMsg>;
type AppExecuteMsg = app::ExecuteMsg<TemplateExecuteMsg>;
type AppQueryMsg = app::QueryMsg<TemplateQueryMsg>;
type AppMigrateMsg = app::MigrateMsg<TemplateMigrateMsg>;

/// Contract wrapper for deploying with BOOT
#[boot_contract(AppInstantiateMsg, AppExecuteMsg, AppQueryMsg, AppMigrateMsg)]
pub struct CounterApp<Chain>;

impl<Chain: BootEnvironment> CounterApp<Chain>
where
    TxResponse<Chain>: IndexResponse,
{
    pub fn new(name: &str, chain: &Chain) -> Self {
        Self(
            Contract::new(name, chain).with_wasm_path(MODULE_NAME),
            // Uncomment to deploy and use contracts with mock implementations
            // .with_mock(Box::new(
            //     ContractWrapper::new_with_empty(
            //         ::contract::execute,
            //         ::contract::instantiate,
            //         ::contract::query,
            //     ),
            // ))
        )
    }

    /// Temporary helper to query the app explicitly
    pub fn query_app<T: Serialize + DeserializeOwned>(
        &self,
        query_msg: TemplateQueryMsg,
    ) -> Result<T, BootError> {
        self.query(&app::QueryMsg::App(query_msg))
    }

    /// Temporary helper to query the app base explicitly
    pub fn query_base<T: Serialize + DeserializeOwned>(
        &self,
        query_msg: BaseQueryMsg,
    ) -> Result<T, BootError> {
        self.query(&app::QueryMsg::Base(query_msg))
    }

    /// Temporary helper to execute the app explicitly
    pub fn execute_app(
        &self,
        execute_msg: TemplateExecuteMsg,
        coins: Option<&[Coin]>,
    ) -> Result<TxResponse<Chain>, BootError> {
        self.execute(&app::ExecuteMsg::App(execute_msg), coins)
    }

    /// Temporary helper to execute the app base explicitly
    pub fn execute_base(
        &self,
        execute_msg: BaseExecuteMsg,
        coins: Option<&[Coin]>,
    ) -> Result<TxResponse<Chain>, BootError> {
        self.execute(&app::ExecuteMsg::Base(execute_msg), coins)
    }
}
