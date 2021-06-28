use crate::gtfs_serde::{deserialize_date, serialize_date};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CalendarDate {
    /// Identifies a set of dates when a service exception occurs for one or more routes.
    /// Each (service_id, date) pair can only appear once in calendar_dates.txt if using calendar.txt and calendar_dates.txt in conjunction.
    /// If a service_id value appears in both calendar.txt and calendar_dates.txt, the information in calendar_dates.txt modifies the service information specified in calendar.txt.
    pub service_id: String,
    #[serde(
        deserialize_with = "deserialize_date",
        serialize_with = "serialize_date"
    )]

    /// Date when service exception occurs.
    pub date: NaiveDate,

    /// Indicates whether service is available on the date specified in the date field.
    pub exception_type: Exception,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum Exception {
    /// Service has been added for the specified date.
    #[serde(rename = "1")]
    Added,

    /// Service has been removed for the specified date.
    #[serde(rename = "2")]
    Deleted,
}
