use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DatalinkVoucherEvent {
    pub reward: u64,
    pub victim_faction: String,
    pub payee_faction: String,
}
