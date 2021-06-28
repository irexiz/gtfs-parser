use crate::gtfs_serde::{deserialize_bool, serialize_bool};
use serde::{Deserialize, Serialize};

use crate::Id;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Attribution {
    /// Identifies an attribution for the dataset, or a subset of it.
    /// This field is useful for translations.
    #[serde(rename = "attribution_id")]
    pub id: Option<String>,

    /// The agency to which the attribution applies.
    /// If one agency_id, route_id, or trip_id attribution is defined, the other fields must be empty.
    /// If none are specified, the attribution applies to the whole dataset.
    pub agency_id: Option<String>,

    /// This field functions in the same way as agency_id, except the attribution applies to a route.
    /// Multiple attributions can apply to the same route.
    pub route_id: Option<String>,

    /// This field functions in the same way as agency_id, except the attribution applies to a trip. Multiple attributions can apply to the same trip.
    pub trip_id: Option<String>,

    /// The name of the organization that the dataset is attributed to.
    pub organization_name: String,

    /// The role of the organization is producer. Allowed values include the following:
    ///   • false or None: Organization doesn’t have this role.
    ///   • true: Organization does have this role.
    /// At least one of the fields, either is_producer, is_operator, or is_authority, must be set to true.
    #[serde(
        deserialize_with = "deserialize_bool",
        serialize_with = "serialize_bool",
        default
    )]
    pub is_producer: bool,

    /// Functions in the same way as is_producer, except the role of the organization is operator.
    #[serde(
        deserialize_with = "deserialize_bool",
        serialize_with = "serialize_bool",
        default
    )]
    pub is_operator: bool,

    /// Functions in the same way as is_producer, except the role of the organization is authority.
    #[serde(
        deserialize_with = "deserialize_bool",
        serialize_with = "serialize_bool",
        default
    )]
    pub is_authority: bool,

    /// The URL of the organization.
    #[serde(rename = "attribution_url")]
    pub url: Option<String>,

    /// The email of the organization.
    #[serde(rename = "attribution_email")]
    pub email: Option<String>,

    /// The phone number of the organization.
    #[serde(rename = "attribution_phone")]
    pub phone: Option<String>,
}

impl Id for Attribution {
    fn id(&self) -> &str {
        match &self.id {
            None => "",
            Some(id) => id,
        }
    }
}
