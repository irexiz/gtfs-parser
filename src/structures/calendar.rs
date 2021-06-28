use crate::{
    gtfs_serde::{deserialize_bool, deserialize_date, serialize_bool, serialize_date},
    Id,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Calendar {
    /// Uniquely identifies a set of dates when service is available for one or more routes.
    /// Each service_id value can appear at most once in a calendar.txt file.
    #[serde(rename = "service_id")]
    pub id: String,

    /// Indicates whether the service operates on all Mondays in the date range specified by the start_date and end_date fields.
    /// Note that exceptions for particular dates may be listed in calendar_dates.txt.
    /// Valid options are:
    ///
    /// true - Service is available for all Mondays in the date range.
    /// false - Service is not available for Mondays in the date range.
    #[serde(
        deserialize_with = "deserialize_bool",
        serialize_with = "serialize_bool"
    )]
    pub monday: bool,

    #[serde(
        deserialize_with = "deserialize_bool",
        serialize_with = "serialize_bool"
    )]
    pub tuesday: bool,

    #[serde(
        deserialize_with = "deserialize_bool",
        serialize_with = "serialize_bool"
    )]
    pub wednesday: bool,

    #[serde(
        deserialize_with = "deserialize_bool",
        serialize_with = "serialize_bool"
    )]
    pub thursday: bool,

    #[serde(
        deserialize_with = "deserialize_bool",
        serialize_with = "serialize_bool"
    )]
    pub friday: bool,

    #[serde(
        deserialize_with = "deserialize_bool",
        serialize_with = "serialize_bool"
    )]
    pub saturday: bool,

    #[serde(
        deserialize_with = "deserialize_bool",
        serialize_with = "serialize_bool"
    )]
    pub sunday: bool,

    /// Start service day for the service interval.
    #[serde(
        deserialize_with = "deserialize_date",
        serialize_with = "serialize_date"
    )]
    pub start_date: NaiveDate,

    /// End service day for the service interval. This service day is included in the interval.
    #[serde(
        deserialize_with = "deserialize_date",
        serialize_with = "serialize_date"
    )]
    pub end_date: NaiveDate,
}

impl Id for Calendar {
    fn id(&self) -> &str {
        &self.id
    }
}
