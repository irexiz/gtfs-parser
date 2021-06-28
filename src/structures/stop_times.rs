use std::sync::Arc;

use crate::gtfs_serde::{
    deserialize_bool, deserialize_option_time, serialize_bool, serialize_option_time,
};
use derivative::Derivative;
use serde::{Deserialize, Serialize};

use super::{routes::ContinuousPickupDropOff, stops::Stop};

#[derive(Debug, Serialize, Deserialize, Default, Derivative)]
pub struct RawStopTime {
    /// Identifies a trip.
    pub trip_id: String,

    /// Arrival time of the stop time.
    /// It's an option since the intermediate stops can have have no arrival
    /// and this arrival needs to be interpolated
    #[serde(
        deserialize_with = "deserialize_option_time",
        serialize_with = "serialize_option_time"
    )]
    pub arrival_time: Option<u64>,

    /// Departure time of the stop time.
    /// It's an option since the intermediate stops can have have no departure
    /// and this departure needs to be interpolated
    #[serde(
        deserialize_with = "deserialize_option_time",
        serialize_with = "serialize_option_time"
    )]
    pub departure_time: Option<u64>,

    /// Identifies the serviced stop.
    /// All stops serviced during a trip must have a record in stop_times.txt.
    pub stop_id: String,

    /// Order of stops for a particular trip.
    /// The values must increase along the trip but do not need to be consecutive.
    pub stop_sequence: u16,

    /// Text that appears on signage identifying the trip's destination to riders.
    /// This field overrides the default trips.trip_headsign when the headsign changes between stops.
    /// If the headsign is displayed for an entire trip, use trips.trip_headsign instead.
    pub stop_headsign: Option<String>,

    /// Indicates pickup method
    #[serde(default)]
    pub pickup_type: PickupDropOffType,

    /// Indicates drop off method.
    #[serde(default)]
    pub drop_off_type: PickupDropOffType,

    /// Indicates whether a rider can board the transit vehicle at any point along the vehicle’s travel path.
    /// The path is described by shapes.txt, from this stop_time to the next stop_time in the trip’s stop_sequence.
    #[serde(default)]
    pub continuous_pickup: ContinuousPickupDropOff,

    /// Indicates whether a rider can alight from the transit vehicle at any point along the vehicle’s travel path as described by shapes.txt,
    /// from this stop_time to the next stop_time in the trip’s stop_sequence.
    #[serde(default)]
    pub continuous_drop_off: ContinuousPickupDropOff,

    /// Actual distance traveled along the associated shape, from the first stop to the stop specified in this record.
    pub shape_dist_traveled: Option<f32>,

    #[serde(
        deserialize_with = "deserialize_bool",
        serialize_with = "serialize_bool",
        default = "default_timepoint"
    )]
    /// Indicates if arrival and departure times for a stop are strictly adhered to by the vehicle or if they are instead approximate and/or interpolated times.
    pub timepoint: bool,
}

fn default_timepoint() -> bool {
    true
}

#[derive(Debug, Default)]
pub struct StopTime {
    pub arrival_time: Option<u64>,
    pub stop: Arc<Stop>,
    pub departure_time: Option<u64>,
    pub pickup_type: PickupDropOffType,
    pub drop_off_type: PickupDropOffType,
    pub stop_sequence: u16,
    pub stop_headsign: Option<String>,
    pub continuous_pickup: ContinuousPickupDropOff,
    pub continuous_drop_off: ContinuousPickupDropOff,
    pub shape_dist_traveled: Option<f32>,
    pub timepoint: bool,
}

impl StopTime {
    pub fn from(stop_time_gtfs: &RawStopTime, stop: Arc<Stop>) -> Self {
        Self {
            arrival_time: stop_time_gtfs.arrival_time,
            departure_time: stop_time_gtfs.departure_time,
            stop,
            pickup_type: stop_time_gtfs.pickup_type,
            drop_off_type: stop_time_gtfs.drop_off_type,
            stop_sequence: stop_time_gtfs.stop_sequence,
            stop_headsign: stop_time_gtfs.stop_headsign.clone(),
            continuous_pickup: stop_time_gtfs.continuous_pickup,
            continuous_drop_off: stop_time_gtfs.continuous_drop_off,
            shape_dist_traveled: stop_time_gtfs.shape_dist_traveled,
            timepoint: stop_time_gtfs.timepoint,
        }
    }
}

#[non_exhaustive]
#[derive(Derivative, Debug, Serialize, Copy, Clone, PartialEq)]
#[derivative(Default(bound = ""))]
pub enum PickupDropOffType {
    #[derivative(Default)]
    #[serde(rename = "0")]
    /// Regularly scheduled pickup/dropoff.
    Regular,

    #[serde(rename = "1")]
    ///  No pickup/dropoff available.
    NotAvailable,

    #[serde(rename = "2")]
    /// Must phone agency to arrange pickup/dropoff.
    ArrangeByPhone,

    #[serde(rename = "3")]
    /// Must coordinate with driver to arrange pickup/dropoff.
    CoordinateWithDriver,
}

impl<'de> Deserialize<'de> for PickupDropOffType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;

        Ok(match s.as_str() {
            "0" => PickupDropOffType::Regular,
            "1" => PickupDropOffType::NotAvailable,
            "2" => PickupDropOffType::ArrangeByPhone,
            "3" => PickupDropOffType::CoordinateWithDriver,
            _ => PickupDropOffType::Regular,
        })
    }
}
