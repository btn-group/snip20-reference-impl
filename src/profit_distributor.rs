use schemars::JsonSchema;
use secret_toolkit::utils::HandleCallback;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ProfitDistributorHandleMsg {
    SetPoolSharesToken { contract_hash: String },
}
impl HandleCallback for ProfitDistributorHandleMsg {
    const BLOCK_SIZE: usize = 256;
}
