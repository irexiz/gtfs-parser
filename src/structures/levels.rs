use serde::{Deserialize, Serialize};

use crate::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct Level {
    /// Id of the level that can be referenced from stops.txt.
    #[serde(rename = "level_id")]
    pub id: String,

    /// Numeric index of the level that indicates relative position of this level in relation to
    /// other levels (levels with higher indices are assumed to be located above levels with lower indices).

    /// Ground level should have index 0, with levels above ground indicated by positive indices
    /// and levels below ground by negative indices.
    #[serde(rename = "level_index")]
    pub index: i64,

    /// Optional name of the level (that matches level lettering/numbering used inside the building or the station).
    /// Is useful for elevator routing (e.g. “take the elevator to level “Mezzanine” or “Platforms” or “-1”).
    #[serde(rename = "level_name")]
    pub name: Option<String>,
}
impl Id for Level {
    fn id(&self) -> &str {
        &self.id
    }
}
