#![feature(conservative_impl_trait, plugin)]
#![plugin(dotenv_macros)]

#[macro_use]
extern crate serde_derive;

extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod error;
mod model;
mod rest;

use std::borrow::Cow;

static DEFAULT_LOCATION: &str = dotenv!("OWM_DEFAULT_LOCATION");
static API_KEY: &str = dotenv!("OWM_API_KEY");

fn main() {
    for query in queries(DEFAULT_LOCATION) {
        println!("query: {}", query);
        match rest::query(query) {
            Err(e) => println!("{}", e),
            Ok(result) => {
                println!(
                    "{}: {:.0} (wind speed: {:.0} mph)",
                    result.city(),
                    result.temperature(),
                    result.wind_speed()
                )
            }
        }
    }
}

fn queries(default: &'static str) -> impl Iterator<Item = Cow<'static, str>> {
    use std::env;
    use std::iter;

    let mut args = env::args().skip(1).map(Cow::from);
    iter::once(args.next().unwrap_or_else(|| Cow::from(default))).chain(args)
}
