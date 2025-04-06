mod model;

use std::{fmt::Display, io};

use clap::Parser;
use model::ApiResponse;

#[derive(Clone, Debug, Parser)]
enum Args {
    City { city: String },
    Zip { zip: String },
    Id { id: String },
}

impl Args {
    fn query(&self) -> Query {
        match self {
            Args::City { city } => Query::City(city),
            Args::Zip { zip: zip_code } => Query::Zip(zip_code),
            Args::Id { id } => Query::Id(id),
        }
    }
}

#[derive(Clone, Debug)]
enum Query<'a> {
    City(&'a str),
    Zip(&'a str),
    Id(&'a str),
}

impl Display for Query<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Query::City(city) => write!(f, "q={city}"),
            Query::Zip(zip) => write!(f, "zip={zip}"),
            Query::Id(id) => write!(f, "id={id}"),
        }
    }
}

fn main() {
    let opts = Args::parse();
    if let Err(e) = run(&opts) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(opts: &Args) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let home = directories::UserDirs::new()
        .map(|dirs| dirs.home_dir().to_owned())
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "user profile not found"))?;

    dotenv::from_path(home.join(".weather.conf"))?;

    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?APPID={}&{}",
        dotenv::var("OWM_API_KEY")?,
        opts.query()
    );

    let weather = match reqwest::blocking::get(url)?.json()? {
        ApiResponse::Weather(weather) => weather,
        ApiResponse::Error(e) => return Err(Box::new(e)),
    };

    println!(
        "{}\n{:0.0} °F\n{}",
        weather.city(),
        weather.temperature(),
        weather.wind()
    );

    Ok(())
}
