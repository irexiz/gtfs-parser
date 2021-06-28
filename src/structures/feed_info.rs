use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedInfo {
    /// Full name of the organization that publishes the dataset.
    /// This might be the same as one of the agency.agency_name values.
    #[serde(rename = "feed_publisher_name")]
    pub name: String,

    /// URL of the dataset publishing organization's website.
    /// This may be the same as one of the agency.agency_url values.
    #[serde(rename = "feed_publisher_url")]
    pub url: String,

    /// Default language for the text in this dataset.
    /// This setting helps GTFS consumers choose capitalization rules and other language-specific settings for the dataset.
    ///
    /// To define another language, use the language field in translations.txt.
    #[serde(rename = "feed_lang")]
    pub lang: String,

    /// Defines the language used when the data consumer doesn’t know the language of the rider.
    /// It's often defined as en, English.
    pub default_lang: Option<String>,

    /// The dataset provides complete and reliable schedule information for service in the period from the beginning of the feed_start_date day to the end of the feed_end_date day.
    /// Both days can be left empty if unavailable.
    ///
    /// The feed_end_date date must not precede the feed_start_date date if both are given.
    /// Dataset providers are encouraged to give schedule data outside this period to advise of likely future service,
    /// but dataset consumers should treat it mindful of its non-authoritative status.
    ///
    /// If feed_start_date or feed_end_date extend beyond the active calendar dates defined in calendar.txt and calendar_dates.txt,
    /// the dataset is making an explicit assertion that there is no service for dates within the feed_start_date to feed_end_date range but not included in the active calendar dates.
    #[serde(rename = "feed_start_date", default)]
    pub start_date: Option<NaiveDate>,

    #[serde(rename = "feed_end_date", default)]
    /// Refer to feed_start_date
    pub end_date: Option<NaiveDate>,

    /// String that indicates the current version of their GTFS dataset.
    /// GTFS-consuming applications can display this value to help dataset publishers determine whether the latest dataset has been incorporated.
    #[serde(rename = "feed_version")]
    pub version: Option<String>,

    /// Email address for communication regarding the GTFS dataset and data publishing practices.
    /// feed_contact_email is a technical contact for GTFS-consuming applications.
    /// Provide customer service contact information through agency.txt.
    #[serde(rename = "feed_contact_email")]
    pub contact_email: Option<String>,

    /// URL for contact information, a web-form, support desk, or other tools for communication
    /// regarding the GTFS dataset and data publishing practices. feed_contact_url is a
    /// technical contact for GTFS-consuming applications. Provide customer service contact
    /// information through agency.txt.
    #[serde(rename = "feed_contact_url")]
    pub contact_url: Option<String>,
}
