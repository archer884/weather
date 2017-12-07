// We deactivate the 'unused' lint on this crate because that seems less destructive than
// removing a lot of model properties that are not currently used by may eventually be used.
#![allow(unused)]

use serde::{Deserialize, Deserializer};
use std::error;
use std::fmt;

#[derive(Deserialize)]
pub struct Weather {
    name: String,

    #[serde(rename = "coord")]
    coords: Coords,

    #[serde(rename = "main")]
    conditions: Conditions,

    wind: Wind,
}

#[derive(Deserialize)]
pub struct Coords {
    lon: f64,
    lat: f64,
}

#[derive(Deserialize)]
pub struct Conditions {
    temp: f32,
    humidity: i32,
    pressure: f32,
    temp_min: f32,
    temp_max: f32,
}

#[derive(Deserialize)]
pub struct Wind {
    speed: f32,
    deg: f32,
}

// TODO: implement a way to get wind direction as well as speed. It would
// probably be best to just create a `Display` implementation for `Wind`
// that does the work--so that we can show things like "2mph NW," etc.
impl Weather {
    /// The name of the nearest city.
    pub fn city(&self) -> &str {
        &self.name
    }

    /// Temperature in Fahrenheit.
    pub fn temperature(&self) -> f32 {
        self.conditions.temp * (9.0 / 5.0) - 459.67
    }

    /// Wind speed in miles per hour.
    pub fn wind_speed(&self) -> f32 {
        self.wind.speed * (11.0 / 25.0)
    }
}

#[derive(Debug)]
pub struct ApiError {
    code: i32,
    message: String,
}

impl<'a> Deserialize<'a> for ApiError {
    fn deserialize<D: Deserializer<'a>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error;

        // This is practically the canonical implementation of my favorite deserialization
        // pattern: start with a template to transform text into something structured, then
        // derive your real data based on that template.
        //
        // In this case, I also make use of a fairly easy mechanism for converting numeric
        // parsing errors into deserialization errors.

        #[derive(Deserialize)]
        struct Template {
            #[serde(rename = "cod")]
            code: String,
            message: String,
        }

        let Template { code, message } = Template::deserialize(d)?;
        Ok(Self {
            code: code.parse().map_err(|e| D::Error::custom(e))?,
            message,
        })
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl error::Error for ApiError {
    fn description(&self) -> &str {
        "An API error occurred"
    }
}

#[cfg(test)]
mod tests {
    use serde_json as json;

    static DATA: &'static str = include_str!("../response.json");

    #[test]
    fn deserialize() {
        json::from_str::<super::Weather>(DATA).unwrap();
    }
}
