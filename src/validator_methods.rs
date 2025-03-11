use ic_cdk::query;
use icrc_ledger_types::icrc1::transfer::TransferArg;

use crate::models::sns_governance::ManageNeuron;

#[query]
pub fn validate_icrc1_transfer(args: TransferArg) -> Result<String, String> {
    Ok(serde_json::to_string(&args).unwrap())
}

#[query]
pub fn validate_manage_neuron(args: ManageNeuron) -> Result<String, String> {
    Ok(serde_json::to_string(&args).unwrap())
}
