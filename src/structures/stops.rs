use derivative::Derivative;
use serde::{Deserialize, Deserializer, Serialize};

use crate::Id;
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Stop {
    /// Identifies a stop, station, or station entrance.
    #[serde(rename = "stop_id")]
    pub id: String,

    /// Short text or a number that identifies the location for riders.
    /// The stop_code can be the same as stop_id if it is public facing.
    #[serde(rename = "stop_code")]
    pub code: Option<String>,

    /// Name of the location. Use a name that people will understand in the local and tourist vernacular.
    #[serde(rename = "stop_name")]
    pub name: Option<String>,

    /// Description of the location that provides useful, quality information.
    #[serde(default, rename = "stop_desc")]
    pub description: Option<String>,

    /// Latitude of the location.
    #[serde(rename = "stop_lat", default)]
    pub latitude: Option<f64>,

    /// Longitude of the location.
    #[serde(rename = "stop_lon", default)]
    pub longitude: Option<f64>,

    /// Identifies the fare zone for a stop.
    pub zone_id: Option<String>,

    /// URL of a web page about the location.
    /// This should be different from the agency.agency_url and the routes.route_url field values.
    #[serde(rename = "stop_url")]
    pub url: Option<String>,

    /// Type of the location
    #[serde(default)]
    pub location_type: StopLocationType,

    /// Defines hierarchy between the different locations defined in stops.txt.
    /// It contains the ID of the parent location
    pub parent_station: Option<String>,

    /// Timezone of the location.
    /// If the location has a parent station, it inherits the parent station’s timezone instead of applying its own.
    /// Stations and parentless stops with empty stop_timezone inherit the timezone specified by agency.agency_timezone.
    #[serde(rename = "stop_timezone")]
    pub timezone: Option<String>,

    /// Indicates whether wheelchair boardings are possible from the location
    #[serde(default)]
    pub wheelchair_boarding: WheelchairBoardingAvailable,

    /// Level of the location. The same level can be used by multiple unlinked stations.
    pub level_id: Option<String>,

    /// Platform identifier for a platform stop (a stop belonging to a station).
    /// This should be just the platform identifier (eg. G or 3).
    /// Words like "platform" or "track" (or the feed’s language-specific equivalent) should not be included.
    pub platform_code: Option<String>,
}

impl Id for Stop {
    fn id(&self) -> &str {
        &self.id
    }
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Derivative)]
#[derivative(Default)]
pub enum StopLocationType {
    #[derivative(Default)]
    StopPoint = 0,
    StopArea = 1,
    StationEntrance = 2,
    GenericNode = 3,
    BoardingArea = 4,
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Derivative)]
#[derivative(Default)]
pub enum WheelchairBoardingAvailable {
    #[serde(rename = "0")]
    #[derivative(Default)]
    /// No accessibility information for the trip.
    InformationNotAvailable,

    #[serde(rename = "1")]
    /// Vehicle being used on this particular trip can accommodate at least one rider in a wheelchair.
    Available,

    #[serde(rename = "2")]
    /// No riders in wheelchairs can be accommodated on this trip.
    NotAvailable,
}

impl<'de> Deserialize<'de> for WheelchairBoardingAvailable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "0" => WheelchairBoardingAvailable::InformationNotAvailable,
            "1" => WheelchairBoardingAvailable::Available,
            "2" => WheelchairBoardingAvailable::NotAvailable,
            _ => WheelchairBoardingAvailable::InformationNotAvailable,
        })
    }
}

impl<'de> Deserialize<'de> for StopLocationType {
    fn deserialize<D>(deserializer: D) -> Result<StopLocationType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "1" => StopLocationType::StopArea,
            "2" => StopLocationType::StationEntrance,
            "3" => StopLocationType::GenericNode,
            "4" => StopLocationType::BoardingArea,
            _ => StopLocationType::StopPoint,
        })
    }
}
