use serde::{Deserialize, Serialize};

/// Fired when in a multi-crew session and the current player changes their role.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeCrewRoleEvent {
    #[serde(default)]
    pub telepresence: bool,
    pub role: ChangeCrewRoleEventRole,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChangeCrewRoleEventRole {
    Idle,
    FireCon,
    FighterCon,
    OnFoot,
    Helm,
}
