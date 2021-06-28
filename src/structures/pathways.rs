use crate::{
    gtfs_serde::{deserialize_bool, serialize_bool},
    Id,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pathway {
    /// The pathway_id field contains an ID that uniquely identifies the pathway.
    /// The pathway_id is used by systems as an internal identifier of this record (e.g., primary key in database),
    /// and therefore the pathway_id must be dataset unique.
    /// Different pathways can go from the same from_stop_id to the same to_stop_id.
    /// For example, this happens when two escalators are side by side in opposite direction,
    /// or when a stair is nearby and elevator and both go from the same place to the same place.
    #[serde(rename = "pathway_id")]
    pub id: String,

    /// Location at which the pathway begins.
    /// It contains a stop_id that identifies a platform, entrance/exit, generic node or boarding area from the stops.txt file.
    pub from_stop_id: String,

    /// Location at which the pathway ends.
    /// It contains a stop_id that identifies a platform, entrance/exit, generic node or boarding area from the stops.txt file.
    pub to_stop_id: String,

    /// Type of pathway between the specified (from_stop_id, to_stop_id) pair.
    #[serde(rename = "pathway_mode")]
    pub mode: PathwayMode,

    /// Indicates in which direction the pathway can be used:
    /// • true: Bidirectional pathway, it can be used in the two directions.
    /// • false: Unidirectional pathway, it can only be used from from_stop_id to to_stop_id.
    ///
    /// Fare gates (pathway_mode=6) and exit gates (pathway_mode=7) cannot be bidirectional.
    #[serde(
        deserialize_with = "deserialize_bool",
        serialize_with = "serialize_bool"
    )]
    pub is_bidirectional: bool,

    /// Horizontal length in meters of the pathway from the origin location (defined in from_stop_id) to the destination location (defined in to_stop_id).
    /// This field is recommended for walkways (pathway_mode=1), fare gates (pathway_mode=6) and exit gates (pathway_mode=7).
    pub length: Option<f64>,

    /// Average time in seconds needed to walk through the pathway from the origin location (defined in from_stop_id) to the destination location (defined in to_stop_id).
    ///
    /// This field is recommended for moving sidewalks (pathway_mode=3), escalators (pathway_mode=4) and elevator (pathway_mode=5).
    pub traversal_time: Option<u64>,

    /// Number of stairs of the pathway.
    /// A positive stair_count implies that the rider walks up from from_stop_id to to_stop_id.
    /// A negative stair_count implies that the rider walks down from from_stop_id to to_stop_id.
    pub stair_count: Option<i64>,

    /// Maximum slope ratio of the pathway. Valid values for this field are:
    /// • 0.0 or None: no slope.
    /// • A float: slope ratio of the pathway, positive for upwards, negative for downwards.
    pub max_slope: Option<f64>,

    /// Minimum width of the pathway in meters.
    pub min_width: Option<f64>,

    /// String of text from physical signage visible to transit riders.
    /// The string can be used to provide text directions to users, such as 'follow signs to '.
    /// The language text should appear in this field exactly how it is printed on the signs - it should not be translated.
    pub signposted_as: Option<String>,

    /// Same than the signposted_as field, but when the pathways is used backward, i.e. from the to_stop_id to the from_stop_id.
    pub reversed_signposted_as: Option<String>,
}

impl Id for Pathway {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum PathwayMode {
    Walkway,
    Stairs,
    Travelator,
    Escalator,
    Elevator,

    ///  A pathway that crosses into an area of the station where a proof of payment is required (usually via a physical payment gate).
    /// Fare gates may either separate paid areas of the station from unpaid ones,
    /// or separate different payment areas within the same station from each other.
    /// This information can be used to avoid routing passengers through stations using shortcuts that would require passengers to make unnecessary payments,
    /// like directing a passenger to walk through a subway platform to reach a busway.
    FareGate,

    ///  Indicates a pathway exiting an area where proof-of-payment is required into an area where proof-of-payment is no longer required.
    ExitGate,
    Other(u16),
}
impl<'de> Deserialize<'de> for PathwayMode {
    fn deserialize<D>(deserializer: D) -> Result<PathwayMode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let i = <u16>::deserialize(deserializer)?;
        Ok(match i {
            1 => PathwayMode::Walkway,
            2 => PathwayMode::Stairs,
            3 => PathwayMode::Travelator,
            4 => PathwayMode::Escalator,
            5 => PathwayMode::Elevator,
            6 => PathwayMode::FareGate,
            7 => PathwayMode::ExitGate,
            other => PathwayMode::Other(other),
        })
    }
}

impl Serialize for PathwayMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            PathwayMode::Walkway => serializer.serialize_u16(1),
            PathwayMode::Stairs => serializer.serialize_u16(2),
            PathwayMode::Travelator => serializer.serialize_u16(3),
            PathwayMode::Escalator => serializer.serialize_u16(4),
            PathwayMode::Elevator => serializer.serialize_u16(5),
            PathwayMode::FareGate => serializer.serialize_u16(6),
            PathwayMode::ExitGate => serializer.serialize_u16(7),
            PathwayMode::Other(a) => serializer.serialize_u16(*a),
        }
    }
}
