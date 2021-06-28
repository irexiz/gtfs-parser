use serde::{Deserialize, Serialize};

use crate::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct FareRule {
    /// Identifies a fare class.
    #[serde(rename = "fare_id")]
    pub id: String,

    /// Identifies a route associated with the fare class.
    /// If several routes with the same fare attributes exist, create a record in fare_rules.txt for each route.
    pub route_id: Option<String>,

    /// Identifies an origin zone.
    /// If a fare class has multiple origin zones, create a record in fare_rules.txt for each origin_id.
    pub origin_id: Option<String>,

    /// Identifies a destination zone.
    /// If a fare class has multiple destination zones, create a record in fare_rules.txt for each destination_id.
    pub destination_id: Option<String>,

    /// Identifies the zones that a rider will enter while using a given fare class.
    /// Used in some systems to calculate correct fare class.
    pub contains_id: Option<String>,
}

impl Id for FareRule {
    fn id(&self) -> &str {
        &self.id
    }
}
