use crate::modules::shared::materials::material::Material;
use crate::modules::shared::materials::material_category::MaterialCategory;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialDiscarded {
    pub category: MaterialCategory,
    pub name: Material,
    pub count: u8,
}