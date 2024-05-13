use serde::{Serialize, Deserialize};

use crate::modules::shared::materials::material::Material;
use crate::modules::shared::odyssey::item::Item;

/// In some cases, the game emits events that refer to materials, but can also contain Odyssey
/// items. If that is the case, this model will be used instead.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum SharedMaterial {
    ShipMaterial(Material),
    OdysseyMaterial(Item),
}
