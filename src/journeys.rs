use chrono::{DateTime, Utc};
use geo::{coord, DensifyHaversine, OutlierDetection, RemoveRepeatedPoints, Simplify};
use geo::{LineString, Point};
use serde::{Deserialize, Serialize};

use crate::points::GpsPoint;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Journey {
    pub index: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub start_city: Option<String>,
    pub end_city: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub flagged: bool,
    pub status: String,
    pub processed: bool,
    pub average_confidence: Option<f64>,
    pub tolerance: Option<f64>,
    pub distance: Option<f64>,
    pub driver_id: Option<String>,
    pub passenger_id: Option<String>,
    pub cancel_reason: Option<String>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub gps_trace: Vec<GpsTrace>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GpsTrace {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub distance: Option<f64>,
    pub hash: Option<String>,
    pub journey_id: String,
    pub user_id: String,
    pub points: Vec<GpsPoint>,
}

#[derive(Clone, Debug)]
pub struct Trace(LineString);

impl Trace {
    pub fn simplified(self) -> Self {
        let line_string = self.as_ref().remove_repeated_points().simplify(&0.00001);

        Self(line_string)
    }
}

impl AsRef<LineString> for Trace {
    fn as_ref(&self) -> &LineString {
        &self.0
    }
}

impl From<&GpsTrace> for Trace {
    fn from(value: &GpsTrace) -> Self {
        let coords: Vec<Point> = value.points.iter().map(Point::from).collect();

        let line_string: LineString = coords
            .iter()
            .zip(coords.outliers(3).iter())
            .filter(|(_, &score)| score <= 1.0) // Adjust threshold as needed
            .map(|(&point, _)| coord! { x: point.x(), y: point.y() })
            .collect();

        let line_string = line_string.densify_haversine(10.0);
        Self(line_string)
    }
}

impl From<GpsTrace> for Trace {
    fn from(value: GpsTrace) -> Self {
        Self::from(&value)
    }
}
