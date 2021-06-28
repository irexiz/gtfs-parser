use serde::{Deserialize, Serialize};

use crate::Id;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Shape {
    /// Identifies a shape.
    #[serde(rename = "shape_id")]
    pub id: String,

    /// Latitude of a shape point.
    /// Each record in shapes.txt represents a shape point used to define the shape.
    #[serde(rename = "shape_pt_lat", default)]
    pub latitude: f64,

    /// Longitude of a shape point.
    #[serde(rename = "shape_pt_lon", default)]
    pub longitude: f64,

    /// Sequence in which the shape points connect to form the shape.
    /// Values must increase along the trip but do not need to be consecutive.
    #[serde(rename = "shape_pt_sequence")]
    pub sequence: usize,

    /// Actual distance traveled along the shape from the first shape point to the point specified in this record.
    /// Used by trip planners to show the correct portion of the shape on a map.
    /// Values must increase along with shape_pt_sequence; they cannot be used to show reverse travel along a route.
    /// Distance units must be consistent with those used in stop_times.txt.
    #[serde(rename = "shape_dist_traveled")]
    pub dist_traveled: Option<f32>,
}

impl Id for Shape {
    fn id(&self) -> &str {
        &self.id
    }
}
