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

#[cfg(test)]
mod tests {
    use serde_json as json;

    static DATA: &'static str = include_str!("../response.json");

    #[test]
    fn deserialize() {
        json::from_str::<super::Weather>(DATA).unwrap();
    }
}
