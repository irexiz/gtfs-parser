use derivative::Derivative;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Transfer {
    /// Identifies a stop or station where a connection between routes begins.
    /// If this field refers to a station, the transfer rule applies to all its child stops.
    pub from_stop_id: String,

    /// Identifies a stop or station where a connection between routes ends.
    /// If this field refers to a station, the transfer rule applies to all child stops.
    pub to_stop_id: String,

    /// Indicates the type of connection for the specified (from_stop_id, to_stop_id) pair.
    pub transfer_type: TransferType,

    /// Amount of time, in seconds, that must be available to permit a transfer between routes at the specified stops.
    /// The min_transfer_time should be sufficient to permit a typical rider to move between the two stops, including buffer time to allow for schedule variance on each route.
    pub min_transfer_time: Option<u64>,
}

#[non_exhaustive]
#[derive(Serialize, Debug, Derivative, PartialEq, Eq, Hash, Clone, Copy)]
#[derivative(Default)]
pub enum TransferType {
    #[derivative(Default)]
    #[serde(rename = "0")]
    /// Recommended transfer point between routes.
    Recommended,

    #[serde(rename = "1")]
    /// Timed transfer point between two routes.
    /// The departing vehicle is expected to wait for the arriving one and leave sufficient time for a rider to transfer between routes.
    Timed,

    #[serde(rename = "2")]
    /// Transfer requires a minimum amount of time between arrival and departure to ensure a connection.
    /// The time required to transfer is specified by min_transfer_time.
    TimedMinimum,

    #[serde(rename = "3")]
    /// Transfers are not possible between routes at the location.
    NotPossible,
}

impl<'de> Deserialize<'de> for TransferType {
    fn deserialize<D>(deserializer: D) -> Result<TransferType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "0" => TransferType::Recommended,
            "1" => TransferType::Timed,
            "2" => TransferType::TimedMinimum,
            "3" => TransferType::NotPossible,
            _ => TransferType::Recommended,
        })
    }
}
