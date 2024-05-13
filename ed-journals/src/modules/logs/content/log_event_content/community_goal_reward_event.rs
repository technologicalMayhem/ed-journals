use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalRewardEvent {
    #[serde(rename = "GCID")]
    pub gcid: u64,
    pub name: String,
    pub system_name: String,
    pub reward: u64,
}
