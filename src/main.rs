#![feature(plugin)]
#![plugin(dotenv_macros)]

#[macro_use]
extern crate serde_derive;

extern crate dotenv;
extern crate hyper;
extern crate or_iter;
extern crate serde_json;

mod client;
mod model;

fn main() {
    use client::WeatherClient;
    use or_iter::OrIter;

    let client = WeatherClient::new();
    let queries = std::env::args().skip(1).or(|| dotenv!("DEFAULT_LOCATION").into());

    for query in queries {
        match client.query(query) {
            Err(e) => println!("{}", e),
            Ok(result) => println!("{}: {:.0}", result.city(), result.temperature()),
        }
    }
}
