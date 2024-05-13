use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CommanderEvent {
    #[serde(rename = "FID")]
    pub fid: String,

    #[serde(rename = "Name")]
    pub name: String,
}
