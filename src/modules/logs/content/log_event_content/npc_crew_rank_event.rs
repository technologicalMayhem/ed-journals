use serde::{Deserialize, Serialize};

use crate::modules::commander::CombatRank;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NPCCrewRankEvent {
    pub npc_crew_name: String,
    pub npc_crew_id: u64,
    pub rank_combat: CombatRank,
}
