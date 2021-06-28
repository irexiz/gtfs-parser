use std::{collections::HashMap, sync::Arc};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{error::Error, to_map, Id};

use super::{
    stop_times::{RawStopTime, StopTime},
    stops::{Stop, WheelchairBoardingAvailable},
};

use derivative::Derivative;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RawTrip {
    /// Identifies a route.
    pub route_id: String,

    #[serde(rename = "trip_id")]
    /// Identifies a trip.
    pub id: String,

    /// Identifies a set of dates when service is available for one or more routes.
    pub service_id: String,

    /// Text that appears on signage identifying the trip's destination to riders.
    #[serde(rename = "trip_headsign")]
    pub headsign: Option<String>,

    /// Public facing text used to identify the trip to riders, for instance, to identify train numbers for commuter rail trips.
    #[serde(rename = "trip_short_name")]
    pub short_name: Option<String>,

    /// Indicates the direction of travel for a trip.
    pub direction_id: Option<Direction>,

    /// Identifies the block to which the trip belongs.
    /// A block consists of a single trip or many sequential trips made using the same vehicle, defined by shared service days and block_id.
    pub block_id: Option<String>,

    /// Identifies a geospatial shape that describes the vehicle travel path for a trip.
    pub shape_id: Option<String>,

    /// Indicates wheelchair accessibility.
    #[serde(default)]
    pub wheelchair_accessible: WheelchairBoardingAvailable,

    /// Indicates whether bikes are allowed.
    #[serde(default)]
    pub bikes_allowed: BikesAllowed,
}

impl Id for RawTrip {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Default)]
pub struct Trip {
    /// Identifies a route.
    pub route_id: String,

    /// Identifies a trip.
    pub id: String,

    /// Identifies a set of dates when service is available for one or more routes.
    pub service_id: String,

    /// Text that appears on signage identifying the trip's destination to riders.
    pub headsign: Option<String>,

    /// Public facing text used to identify the trip to riders, for instance, to identify train numbers for commuter rail trips.
    pub short_name: Option<String>,

    /// Indicates the direction of travel for a trip.
    pub direction_id: Option<Direction>,

    /// Identifies the block to which the trip belongs.
    /// A block consists of a single trip or many sequential trips made using the same vehicle, defined by shared service days and block_id.
    pub block_id: Option<String>,

    /// Identifies a geospatial shape that describes the vehicle travel path for a trip.
    pub shape_id: Option<String>,

    /// Indicates wheelchair accessibility.
    pub wheelchair_accessible: WheelchairBoardingAvailable,

    /// Indicates whether bikes are allowed.
    pub bikes_allowed: BikesAllowed,

    /// Linked stop times based off of stop_times.txt and stops.txt
    pub stop_times: Vec<StopTime>,
}

impl Id for Trip {
    fn id(&self) -> &str {
        &self.id
    }
}

impl From<RawTrip> for Trip {
    fn from(rt: RawTrip) -> Self {
        Self {
            route_id: rt.route_id,
            id: rt.id,
            service_id: rt.service_id,
            headsign: rt.headsign,
            short_name: rt.short_name,
            direction_id: rt.direction_id,
            block_id: rt.block_id,
            shape_id: rt.shape_id,
            wheelchair_accessible: rt.wheelchair_accessible,
            bikes_allowed: rt.bikes_allowed,
            stop_times: vec![],
        }
    }
}

impl Trip {
    pub fn create_trips(
        raw_trips: Vec<RawTrip>,
        raw_stop_times: Vec<RawStopTime>,
        stops: &HashMap<String, Arc<Stop>>,
    ) -> Result<Vec<Trip>, Error> {
        let mut trips = to_map(raw_trips.into_iter().map(|rt| Trip {
            id: rt.id,
            service_id: rt.service_id,
            route_id: rt.route_id,
            stop_times: vec![],
            shape_id: rt.shape_id,
            headsign: rt.headsign,
            short_name: rt.short_name,
            direction_id: rt.direction_id,
            block_id: rt.block_id,
            wheelchair_accessible: rt.wheelchair_accessible,
            bikes_allowed: rt.bikes_allowed,
        }));

        for raw_stop_time in raw_stop_times {
            let trip = &mut trips
                .get_mut(&raw_stop_time.trip_id)
                .ok_or_else(|| Error::ReferenceError(raw_stop_time.trip_id.to_string()))?;

            let stop = stops
                .get(&raw_stop_time.stop_id)
                .ok_or_else(|| Error::ReferenceError(raw_stop_time.stop_id.to_string()))?;

            trip.stop_times
                .push(StopTime::from(&raw_stop_time, Arc::clone(&stop)));
        }

        for trip in &mut trips.values_mut() {
            trip.stop_times
                .sort_by(|a, b| a.stop_sequence.cmp(&b.stop_sequence));
        }

        let trips = trips.into_iter().map(|(_key, value)| value).collect_vec();

        Ok(trips)
    }
}

#[non_exhaustive]
#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
pub enum Direction {
    /// Travel in one direction (e.g. outbound travel).
    #[serde(rename = "0")]
    Outbound,

    #[serde(rename = "1")]
    /// Travel in the opposite direction (e.g. inbound travel).
    Inbound,
}

#[non_exhaustive]
#[derive(Debug, Derivative, Serialize, Copy, Clone, PartialEq)]
#[derivative(Default)]
pub enum BikesAllowed {
    #[derivative(Default)]
    #[serde(rename = "0")]
    /// No bike information for the trip.
    NoBikeInfo,

    #[serde(rename = "1")]
    /// Vehicle being used on this particular trip can accommodate at least one bicycle.
    AtLeastOneBike,

    #[serde(rename = "2")]
    /// No bicycles are allowed on this trip.
    NoBikesAllowed,
}

impl<'de> Deserialize<'de> for BikesAllowed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;

        Ok(match s.as_str() {
            "0" => BikesAllowed::NoBikeInfo,
            "1" => BikesAllowed::AtLeastOneBike,
            "2" => BikesAllowed::NoBikesAllowed,
            _ => BikesAllowed::NoBikeInfo,
        })
    }
}
