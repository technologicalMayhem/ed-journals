use crate::modules::shared::ship::ship_type::ShipType;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardSellEvent {
    pub ship_type: ShipType,

    #[serde(rename = "SellShipID")]
    pub sell_ship_id: u8,
    pub ship_price: u64,
    pub system: String,
}
