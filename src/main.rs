#![feature(plugin)]
#![plugin(dotenv_macros)]

#[macro_use]
extern crate serde_derive;

extern crate dotenv;
extern crate hyper;
extern crate serde_json;

mod client;
mod iter;
mod model;

fn main() {
    use client::WeatherClient;
    use iter::DefaultIter;

    let client = WeatherClient::new();
    let queries = std::env::args().skip(1).default(|| dotenv!("DEFAULT_LOCATION").into());

    for query in queries {
        match client.query(query) {
            Err(e) => println!("Bad query: {}", e),
            Ok(result) => println!("{}: {:.0}", result.city(), result.temperature()),
        }
    }
}
