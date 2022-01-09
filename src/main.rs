use google_maps::directions::{Location, TravelMode};
use google_maps::prelude::Duration;
use google_maps::ClientSettings;
use std::fs::File;

use std::error::Error;
use std::io::{BufRead, BufReader};

use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::ops::Add;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Opt {
    Search {
        /// The string to search with the google maps direction api
        query: String,
    },
    /// Search for all the addresses in a file. Each line in the input should be an address.
    Summarize { file: PathBuf },
}

async fn summarize_direction_time(
    client: &ClientSettings,
    a: Location,
    b: Location,
) -> Option<String> {
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

async fn search_and_summarize(client: &ClientSettings, address: &str) -> Option<String> {
    // Microsoft Studio C
    let dustin_work_address: Location =
        Location::PlaceId(String::from("ChIJz85LumxtkFQRhW-lYWwmRpM"));
    // Boeing 40-87
    let valery_work_address: Location =
        Location::PlaceId(String::from("ChIJRe_JoxEBkFQRbaakkmkDFk0"));

    let address_location = Location::Address(address.to_string());

    Some(format!(
        "Dustin -> {}: {}\nValery -> {}: {}",
        address,
        summarize_direction_time(client, dustin_work_address, address_location.clone()).await?,
        address,
        summarize_direction_time(client, valery_work_address, address_location).await?
    ))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const API_KEY: &str = include_str!("../api_key.txt");

    let google_maps_client = ClientSettings::new(API_KEY);

    let opt = Opt::from_args();

    match opt {
        Opt::Search { query } => {
            println!(
                "{}",
                search_and_summarize(&google_maps_client, &query)
                    .await
                    .ok_or(format!("Couldn't summarize for query: {}", query))?
            )
        }
        Opt::Summarize { file } => {
            let file = File::open(file).expect("Couldn't open file");
            let reader = BufReader::new(file);
            let mut futures = FuturesUnordered::new();
            for address in reader.lines() {
                if let Ok(address) = address {
                    futures.push(search_and_summarize(&google_maps_client, address))
                }
            }
            loop {
                match futures.next().await {
                    Some(Some(summary)) => {
                        println!("{}", summary)
                    },
                    None => {
                        break;
                    },
                    _ => ()
                }
            }
        }
    };
    Ok(())
}
