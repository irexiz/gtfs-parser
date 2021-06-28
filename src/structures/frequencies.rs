use crate::gtfs_serde::{deserialize_time, serialize_time};
use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Frequency {
    /// Identifies a trip to which the specified headway of service applies.
    pub trip_id: String,

    /// Time at which the first vehicle departs from the first stop of the trip with the specified headway.
    #[serde(
        deserialize_with = "deserialize_time",
        serialize_with = "serialize_time"
    )]
    pub start_time: u64,

    /// Time at which service changes to a different headway (or ceases) at the first stop in the trip.
    #[serde(
        deserialize_with = "deserialize_time",
        serialize_with = "serialize_time"
    )]
    pub end_time: u64,

    /// Time, in seconds, between departures from the same stop (headway) for the trip, during the time interval specified by start_time and end_time.
    /// Multiple headways for the same trip are allowed, but may not overlap.
    /// New headways may start at the exact time the previous headway ends.
    pub headway_secs: u64,

    /// Indicates the type of service
    #[serde(default)]
    pub exact_times: ServiceType,
}

#[non_exhaustive]
#[derive(Derivative, Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
#[derivative(Default)]
pub enum ServiceType {
    #[serde(rename = "0")]
    #[derivative(Default)]
    /// Frequency-based trips.
    FrequencyBased,

    #[serde(rename = "1")]
    /// Schedule-based trips with the exact same headway throughout the day.
    /// In this case the end_time value must be greater than the last desired trip start_time but less than the last desired trip start_time + headway_secs.
    ScheduleBased,
}
