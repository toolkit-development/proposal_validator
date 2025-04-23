use candid::Principal;
use ic_cdk::query;
use icrc_ledger_types::{icrc1::transfer::TransferArg, icrc2::approve::ApproveArgs};

use crate::models::sns_governance::ManageNeuron;

#[query]
pub fn validate_icrc1_transfer(args: TransferArg) -> Result<String, String> {
    Ok(serde_json::to_string(&args).unwrap())
}

#[query]
pub fn validate_icrc2_approve(args: ApproveArgs) -> Result<String, String> {
    Ok(serde_json::to_string(&args).unwrap())
}

#[query]
pub fn validate_manage_neuron(args: ManageNeuron) -> Result<String, String> {
    Ok(serde_json::to_string(&args).unwrap())
}

#[query]
pub fn validate_icpswap_transfer_position(
    arg0: Principal,
    arg1: Principal,
    arg2: candid::Nat,
) -> Result<String, String> {
    Ok(serde_json::to_string(&(arg0, arg1, arg2)).unwrap())
}
