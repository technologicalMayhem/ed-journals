use serde::{Serialize, Deserialize};

use crate::modules::shared::odyssey::item::Item;
use crate::modules::shared::odyssey::item_type::ItemType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DropItemsEvent {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    #[serde(rename = "Type")]
    pub kind: ItemType,

    #[serde(rename = "OwnerID")]
    pub owner_id: u64,
    pub count: u16,
}
