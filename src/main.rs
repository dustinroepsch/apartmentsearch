#![deny(clippy::pedantic)]

use google_maps::directions::{Location, TravelMode};
use google_maps::prelude::Duration;
use google_maps::ClientSettings;

use std::fs::File;
use std::future::Future;

use std::io::{BufRead, BufReader};

use anyhow::{anyhow, Context, Error};
use futures::stream::FuturesUnordered;
use futures::{FutureExt, StreamExt};
use lazy_static::lazy_static;
use starting_point::{StartingPoint, StartingPoints, DEFAULT_STARTING_POINTS};
use std::ops::Add;
use std::path::PathBuf;

use structopt::StructOpt;

mod starting_point;

const API_KEY: &str = include_str!("../api_key.txt");

lazy_static! {
    static ref CLIENT_SETTINGS: ClientSettings = ClientSettings::new(API_KEY);
}

#[derive(Debug, StructOpt)]
enum Opt {
    /// Search one address against a list of starting point
    Search {
        /// The address to test every starting point against
        address: String,

        #[structopt(
            short, long,
            default_value = DEFAULT_STARTING_POINTS,
        )]
        /// The starting points that will be tested. Formatted as a comma separated list of place_id:nickname elements
        starting_points: StartingPoints,
    },
    /// Run the search command for every address in a file
    Summarize {
        /// The file which should contain one address per line
        file: PathBuf,
        #[structopt(
            short, long,
            default_value = DEFAULT_STARTING_POINTS,
        )]
        /// The starting points that will be tested against every address in the input file. Formatted as a comma separated list of place_id:nickname elements
        starting_points: StartingPoints,
    },
}

async fn summarize_direction_time(a: Location, b: Location) -> Result<String, Error> {
    let response = CLIENT_SETTINGS
        .directions(a.clone(), b.clone())
        .with_travel_mode(TravelMode::Driving)
        .execute()
        .await
        .with_context(|| {
            format!(
                "Error getting directions from {:?} to {:?}.",
                a.clone(),
                b.clone()
            )
        })?;

    let route = response.routes.first().ok_or(anyhow!(
        "The list of routes returned from google was empty."
    ))?;

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

fn search_and_summarize(
    starting_points: Vec<StartingPoint>,
    ending_address: &str,
) -> Vec<impl Future<Output = Result<String, Error>>> {
    let ending_location = Location::Address(ending_address.to_string());

    let mut futures: Vec<_> = Vec::new();

    for starting_point in starting_points {
        let starting_location = Location::PlaceId(starting_point.place_id);
        let starting_name = starting_point.display_name.clone();
        let ending_address = ending_address.to_string();
        let future = summarize_direction_time(starting_location, ending_location.clone());
        futures.push(future.map(move |result| {
            result.map(|time_summary| {
                format!("{} -> {}: {}", starting_name, ending_address, time_summary)
            })
        }));
    }

    futures
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let mut futures: FuturesUnordered<_> = match opt {
        Opt::Search {
            address: query,
            starting_points,
        } => FuturesUnordered::from_iter(search_and_summarize(starting_points.into(), &query)),
        Opt::Summarize {
            file,
            starting_points,
        } => {
            let file = File::open(file).expect("Couldn't open file");
            let flattened: Vec<_> = BufReader::new(file)
                .lines()
                .filter_map(std::result::Result::ok)
                .flat_map(|address| search_and_summarize(starting_points.clone().into(), &address))
                .collect();
            FuturesUnordered::from_iter(flattened)
        }
    };

    let mut errors: Vec<Error> = Vec::new();

    while let Some(result) = futures.next().await {
        match result {
            Ok(summary) => {
                println!("{}", summary);
            }
            Err(error) => {
                errors.push(error);
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        let mut error = anyhow!("There were errors while executing tasks.");
        while let Some(context_error) = errors.pop() {
            error = error.context(context_error);
        }

        Err(error)
    }
}
