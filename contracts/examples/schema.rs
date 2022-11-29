use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for, write_api};

use counter_app::contract::CounterApp;
use counter_app::msg::{ConfigResponse, TemplateExecuteMsg, TemplateInstantiateMsg, TemplateMigrateMsg, TemplateQueryMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    CounterApp::export_schema(&out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);

    write_api! {
        instantiate: TemplateInstantiateMsg,
        query: TemplateQueryMsg,
        execute: TemplateExecuteMsg,
        migrate: TemplateMigrateMsg,
    };
}
