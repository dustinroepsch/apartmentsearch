#![deny(clippy::pedantic)]

use google_maps::directions::{Location, TravelMode};
use google_maps::prelude::Duration;
use google_maps::ClientSettings;
use std::fs::File;

use std::io::{BufRead, BufReader};

use anyhow::Error;
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
    /// Search for all the addresses in a file
    Summarize {
        /// The file which should contain one address per line
        file: PathBuf,
    },
}

async fn summarize_direction_time(
    client: &ClientSettings,
    a: Location,
    b: Location,
) -> Result<String, Error> {
    let response = client
        .directions(a.clone(), b.clone())
        .with_travel_mode(TravelMode::Driving)
        .execute()
        .await?;

    let route = response
        .routes
        .first()
        .ok_or_else(|| return Ok::<String, Error>(format!("No route from {:?} to {:?}", a, b)))
        .unwrap();

    let duration: Duration = route
        .legs
        .iter()
        .map(|leg| leg.duration.value)
        .fold(Duration::zero(), Duration::add);
    let hours = duration.num_hours();
    let mut minutes = duration.num_minutes() % 60;
    //round up if we are less than thirty seconds from minute + 1
    if duration.num_seconds() % 60 > 30 {
        minutes += 1;
    }

    Ok(format!(
        "Total time: {} hours and {} minutes ",
        hours, minutes
    ))
}

async fn search_and_summarize(client: &ClientSettings, address: String) -> Result<String, Error> {
    // Microsoft Studio C
    let dustin_work_address: Location =
        Location::PlaceId(String::from("ChIJz85LumxtkFQRhW-lYWwmRpM"));
    // Boeing 40-87
    let valery_work_address: Location =
        Location::PlaceId(String::from("ChIJRe_JoxEBkFQRbaakkmkDFk0"));

    let address_location = Location::Address(address.to_string());

    Ok(format!(
        "Dustin -> {}: {}\nValery -> {}: {}",
        address,
        summarize_direction_time(client, dustin_work_address, address_location.clone()).await?,
        address,
        summarize_direction_time(client, valery_work_address, address_location).await?
    ))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const API_KEY: &str = include_str!("../api_key.txt");

    let google_maps_client = ClientSettings::new(API_KEY);

    let opt = Opt::from_args();

    match opt {
        Opt::Search { query } => {
            println!(
                "{}",
                search_and_summarize(&google_maps_client, query.clone()).await?
            );
        }
        Opt::Summarize { file } => {
            let file = File::open(file).expect("Couldn't open file");
            let reader = BufReader::new(file);
            let mut futures: FuturesUnordered<_> = reader
                .lines()
                .filter_map(std::result::Result::ok)
                .map(|address| search_and_summarize(&google_maps_client, address))
                .collect();

            while let Some(result) = futures.next().await {
                match result {
                    Ok(summary) => {
                        println!("{}", summary);
                    }
                    Err(error) => {
                        eprintln!("{}", error);
                    }
                }
            }
        }
    };
    Ok(())
}
