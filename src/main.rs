use google_maps::directions::{Location, TravelMode};
use google_maps::prelude::Duration;
use google_maps::{ClientSettings, LatLng};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::error::Error;
use std::ops::Add;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Opt {
    /// Search with an arbitrary string
    Search {
        /// The string to search with the google maps direction api
        query: String,
    },
    /// Search with a lat long
    LatLong { lat: Decimal, long: Decimal },
}

async fn summarize(client: &ClientSettings, a: Location, b: Location) -> Option<String> {
    let route = client
        .directions(a, b)
        .with_travel_mode(TravelMode::Driving)
        .execute()
        .await
        .ok()?
        .routes
        .first()?
        .clone();

    let duration: Duration = route
        .legs
        .iter()
        .map(|leg| leg.duration.value)
        .fold(Duration::zero(), Duration::add);
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    Some(format!("Total time: {}:{} ", hours, minutes))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const API_KEY: &str = include_str!("../api_key.txt");

    let dustin_work_address: Location =
        Location::LatLng(LatLng::try_from(dec!(47.6430149), dec!(-122.1402896))?);

    let valery_work_address: Location =
        Location::LatLng(LatLng::try_from(dec!(47.9300239), dec!(-122.65863))?);

    let opt = Opt::from_args();

    let apartment_address = match opt {
        Opt::Search { query } => (Location::Address(query)),
        Opt::LatLong { lat, long } => (Location::LatLng(LatLng::try_from(lat, long)?)),
    };

    let google_maps_client = ClientSettings::new(API_KEY);

    println!("Valery Work to Dustin Work: ");
    println!(
        "{}",
        summarize(
            &google_maps_client,
            valery_work_address.clone(),
            dustin_work_address.clone()
        )
        .await
        .ok_or("Failed to look up directions")?
    );
    println!("Valery Work to Apartment: ");
    println!(
        "{}",
        summarize(
            &google_maps_client,
            valery_work_address,
            apartment_address.clone()
        )
        .await
        .ok_or("Failed to look up directions")?
    );
    println!("Dustin Work to Apartment: ");
    println!(
        "{}",
        summarize(&google_maps_client, dustin_work_address, apartment_address)
            .await
            .ok_or("Failed to look up directions")?
    );

    Ok(())
}
