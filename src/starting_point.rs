use anyhow::{anyhow, Context, Error};
use derive_more::{From, Into};

use std::str::FromStr;

pub const DEFAULT_STARTING_POINTS: &str =
    "ChIJz85LumxtkFQRhW-lYWwmRpM:Microsoft,ChIJRe_JoxEBkFQRbaakkmkDFk0:Boeing";

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StartingPoint {
    /// A google place id
    pub place_id: String,
    /// The display name to show in the output
    pub display_name: String,
}

impl StartingPoint {
    pub fn new(place_id: &str, display_name: &str) -> Self {
        Self {
            place_id: place_id.to_string(),
            display_name: display_name.to_string(),
        }
    }
}

impl FromStr for StartingPoint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (place_id, display_name) = s.split_once(':').context(anyhow!(
            "Error splitting \"{}\" exactly once on the delimiter ':'.",
            s
        ))?;
        Ok(StartingPoint::new(place_id, display_name))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, From, Into)]
pub struct StartingPoints(Vec<StartingPoint>);

impl FromStr for StartingPoints {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let point_parse_results: Vec<_> = s.split(',').map(str::parse).collect();

        let mut points = Vec::new();
        let mut errors = Vec::new();

        for result in point_parse_results {
            match result {
                Ok(point) => points.push(point),
                Err(err) => errors.push(err),
            }
        }

        if errors.is_empty() {
            Ok(StartingPoints(points))
        } else {
            let mut error = anyhow!("All of these errors occured while executing tasks.");
            while let Some(next_error) = errors.pop() {
                error = error.context(next_error);
            }
            Err(error)
        }
    }
}
