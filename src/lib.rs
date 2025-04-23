pub mod models;
pub mod validator_methods;

use candid::export_service;
use ic_cdk::query;

#[query]
fn icts_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

#[query]
fn icts_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[query]
fn icts_description() -> String {
    env!("CARGO_PKG_DESCRIPTION").to_string()
}

#[query]
pub fn __get_candid_interface_tmp_hack() -> String {
    use crate::models::sns_governance::ManageNeuron;
    use candid::Principal;
    use icrc_ledger_types::icrc1::transfer::TransferArg;
    use icrc_ledger_types::icrc2::approve::ApproveArgs;
    export_service!();
    __export_service()
}

#[test]
pub fn candid() {
    use std::env;
    use std::fs::write;
    use std::path::PathBuf;

    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    write(
        dir.join("proposal_validator.did"),
        __get_candid_interface_tmp_hack(),
    )
    .expect("Write failed.");
}
