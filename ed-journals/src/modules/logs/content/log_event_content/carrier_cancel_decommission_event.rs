use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierCancelDecommissionEvent {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
}
