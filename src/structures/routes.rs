use crate::{
    gtfs_serde::{deserialize_option_color, serialize_option_color},
    Id,
};
use derivative::Derivative;
use rgb::RGB8;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Route {
    /// Identifies a route.
    #[serde(rename = "route_id")]
    pub id: String,

    /// Agency for the specified route.
    /// This field is required when the dataset provides data for routes from more than one agency in agency.txt, otherwise it is optional.
    pub agency_id: Option<String>,

    /// Short name of a route.
    /// This will often be a short, abstract identifier like "32", "100X", or "Green" that riders use to identify a route, but which doesn't give any indication of what places the route serves.
    /// Either route_short_name or route_long_name must be specified, or potentially both if appropriate.
    #[serde(rename = "route_short_name")]
    pub short_name: String,

    /// Full name of a route.
    /// This name is generally more descriptive than the route_short_name and often includes the route's destination or stop.
    /// Either route_short_name or route_long_name must be specified, or potentially both if appropriate.
    #[serde(rename = "route_long_name")]
    pub long_name: String,

    /// Description of a route that provides useful, quality information.
    /// Do not simply duplicate the name of the route.
    #[serde(rename = "route_desc")]
    pub desc: Option<String>,

    /// Indicates the type of transportation used on a route.
    pub route_type: RouteType,

    /// URL of a web page about the particular route.
    /// Should be different from the agency.agency_url value.
    #[serde(rename = "route_url")]
    pub url: Option<String>,

    /// Route color designation that matches public facing material.
    /// Defaults to white (FFFFFF) when omitted or left empty.
    /// The color difference between route_color and route_text_color should provide sufficient contrast when viewed on a black and white screen.
    #[serde(
        deserialize_with = "deserialize_option_color",
        serialize_with = "serialize_option_color",
        default = "white_rgb"
    )]
    pub route_color: Option<RGB8>,

    /// Legible color to use for text drawn against a background of route_color.
    /// Defaults to black (000000) when omitted or left empty.
    /// The color difference between route_color and route_text_color should provide sufficient contrast when viewed on a black and white screen.
    #[serde(
        deserialize_with = "deserialize_option_color",
        serialize_with = "serialize_option_color",
        default = "black_rgb"
    )]
    pub route_text_color: Option<RGB8>,

    /// Orders the routes in a way which is ideal for presentation to customers.
    /// Routes with smaller route_sort_order values should be displayed first.
    #[serde(rename = "route_sort_order")]
    pub sort_order: Option<u32>,

    /// Indicates whether a rider can board the transit vehicle anywhere along the vehicle’s travel path.
    /// The path is described by shapes.txt on every trip of the route.
    #[serde(default)]
    pub continuous_pickup: ContinuousPickupDropOff,

    /// Indicates whether a rider can alight from the transit vehicle at any point along the vehicle’s travel path.
    /// The path is described by shapes.txt on every trip of the route.
    #[serde(default)]
    pub continuous_drop_off: ContinuousPickupDropOff,
}

const fn black_rgb() -> Option<RGB8> {
    Some(RGB8 { r: 0, g: 0, b: 0 })
}

const fn white_rgb() -> Option<RGB8> {
    Some(RGB8 {
        r: 255,
        g: 255,
        b: 255,
    })
}

impl Id for Route {
    fn id(&self) -> &str {
        &self.id
    }
}

#[non_exhaustive]
#[derive(Derivative)]
#[derivative(Default)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RouteType {
    #[derivative(Default)]
    Bus,

    Tramway,
    Subway,
    Rail,
    Ferry,
    CableCar,
    Gondola,
    Funicular,
    // extended GTFS (https://developers.google.com/transit/gtfs/reference/extended-route-types)
    Coach,
    Air,
    Taxi,
    Other(u16),
}

#[non_exhaustive]
#[derive(Derivative, Debug, Serialize, Copy, Clone, PartialEq)]
#[derivative(Default)]
pub enum ContinuousPickupDropOff {
    #[serde(rename = "0")]
    /// Continuous stopping pickup.
    Continuous,

    #[serde(rename = "1")]
    #[derivative(Default)]
    /// No continuous stopping pickup.
    NotAvailable,

    #[serde(rename = "2")]
    /// Must phone an agency to arrange continuous pickup.
    ArrangeByPhone,

    #[serde(rename = "3")]
    /// Must coordinate with a driver to arrange continuous stopping pickup.
    CoordinateWithDriver,
}

impl<'de> Deserialize<'de> for ContinuousPickupDropOff {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;

        Ok(match s.as_str() {
            "0" => ContinuousPickupDropOff::Continuous,
            "1" => ContinuousPickupDropOff::NotAvailable,
            "2" => ContinuousPickupDropOff::ArrangeByPhone,
            "3" => ContinuousPickupDropOff::CoordinateWithDriver,
            _ => ContinuousPickupDropOff::NotAvailable,
        })
    }
}

impl<'de> Deserialize<'de> for RouteType {
    fn deserialize<D>(deserializer: D) -> Result<RouteType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let i = u16::deserialize(deserializer)?;

        let hundreds = i / 100;
        Ok(match (i, hundreds) {
            (0, _) | (_, 9) => RouteType::Tramway,
            (1, _) | (_, 4) => RouteType::Subway,
            (2, _) | (_, 1) => RouteType::Rail,
            (3, _) | (_, 7) | (_, 8) => RouteType::Bus,
            (4, _) | (_, 10) | (_, 12) => RouteType::Ferry,
            (5, _) => RouteType::CableCar,
            (6, _) | (_, 13) => RouteType::Gondola,
            (7, _) | (_, 14) => RouteType::Funicular,
            (_, 2) => RouteType::Coach,
            (_, 11) => RouteType::Air,
            (_, 15) => RouteType::Taxi,
            _ => RouteType::Other(i),
        })
    }
}

impl Serialize for RouteType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // NOTE: for extended route type, we might lose the initial precise route type
        serializer.serialize_u16(match self {
            RouteType::Tramway => 0,
            RouteType::Subway => 1,
            RouteType::Rail => 2,
            RouteType::Bus => 3,
            RouteType::Ferry => 4,
            RouteType::CableCar => 5,
            RouteType::Gondola => 6,
            RouteType::Funicular => 7,
            RouteType::Coach => 200,
            RouteType::Air => 1100,
            RouteType::Taxi => 1500,
            RouteType::Other(i) => *i,
        })
    }
}
