use ic_cdk::{caller, query};
use icrc_ledger_types::icrc1::transfer::TransferArg;

use crate::models::sns_governance::ManageNeuron;

#[query]
fn icts_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

#[query]
fn icts_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[query]
pub fn validate_icrc_transfer(args: TransferArg) -> Result<String, String> {
    let text = format!(
        "transfer of {} tokens from {} to {}.",
        args.amount,
        caller(),
        args.to
    );
    Ok(text)
}

#[query]
pub fn validate_manage_neuron(args: ManageNeuron) -> Result<String, String> {
    let text = format!(
        "manage neuron {:?} with {:?}.",
        args.subaccount, args.command
    );
    Ok(text)
}
