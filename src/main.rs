
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
fn main() {
    const API_KEY: &str = include_str!("../api_key.txt");
    let opt = Opt::from_args();
    println!("{:?}", opt)
}
