use serde::{Deserialize, Serialize};

use crate::Id;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Agency {
    /// Identifies a transit brandwhich is often synonymous with a transit agency.
    /// Note that in some cases, such as when a single agency operates multiple separate services, agencies and brands are distinct.
    /// Google GTFS reference document uses the term "agency" in place of "brand".
    /// A dataset may contain data from multiple agencies.
    ///
    /// This field is required when the dataset contains data for multiple transit agencies, otherwise it is optional.
    #[serde(rename = "agency_id")]
    pub id: Option<String>,

    /// Full name of the transit agency.
    #[serde(rename = "agency_name")]
    pub name: String,

    /// URL of the transit agency.
    #[serde(rename = "agency_url")]
    pub url: String,

    /// Timezone where the transit agency is located.
    /// If multiple agencies are specified in the dataset, each must have the same agency_timezone.
    #[serde(rename = "agency_timezone")]
    pub timezone: String,

    /// Primary language used by this transit agency.
    /// This field helps GTFS consumers choose capitalization rules and other language-specific settings for the dataset.
    #[serde(rename = "agency_lang")]
    pub lang: Option<String>,

    /// A voice telephone number for the specified agency.
    /// This field is a string value that presents the telephone number as typical for the agency's service area.
    /// It can and should contain punctuation marks to group the digits of the number.
    /// Dialable text (for example, TriMet's 503-238-RIDE) is permitted, but the field must not contain any other descriptive text.
    #[serde(rename = "agency_phone")]
    pub phone: Option<String>,

    /// URL of a web page that allows a rider to purchase tickets or other fare instruments for that agency online.
    #[serde(rename = "agency_fare_url")]
    pub fare_url: Option<String>,

    /// Email address actively monitored by the agency’s customer service department.
    /// This email address should be a direct contact point where transit riders can reach a customer service representative at the agency.
    #[serde(rename = "agency_email")]
    pub email: Option<String>,
}

impl Id for Agency {
    fn id(&self) -> &str {
        match &self.id {
            None => "",
            Some(id) => id,
        }
    }
}
