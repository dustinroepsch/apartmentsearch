use anyhow::Error;
use std::str::FromStr;

pub const DEFAULT_STARTING_POINTS: &str =
    "ChIJz85LumxtkFQRhW-lYWwmRpM:Microsoft ChIJRe_JoxEBkFQRbaakkmkDFk0:Boeing";

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
        let (place_id, display_name) = s.split_once(':').unwrap();
        Ok(StartingPoint::new(place_id, display_name))
    }
}
