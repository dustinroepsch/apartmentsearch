use google_maps::directions::{Location, TravelMode};
use google_maps::{ClientSettings, LatLng};
use rust_decimal_macros::dec;
use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Opt {
    /// Search with an arbitrary string
    Search {
        /// The string to search with the google maps direction api
        query: String,
    },
    /// Search with a lat long
    LatLong {},
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const API_KEY: &str = include_str!("../api_key.txt");

    let dustin_work_address: Location =
        Location::LatLng(LatLng::try_from(dec!(47.6430149), dec!(-122.1402896))?);

    let valery_work_address: Location =
        Location::LatLng(LatLng::try_from(dec!(47.9300239), dec!(-122.65863))?);

    let google_maps_client = ClientSettings::new(API_KEY);

    let response = google_maps_client
        .directions(dustin_work_address, valery_work_address)
        .with_travel_mode(TravelMode::Driving)
        .execute()
        .await?;
    println!("{:#?}", response);

    let _opt = Opt::from_args();

    Ok(())
}
