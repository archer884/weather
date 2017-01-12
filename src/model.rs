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

impl Weather {
    pub fn city(&self) -> &str {
        &self.name
    }

    pub fn temperature(&self) -> f32 {
        self.conditions.temp * (9.0 / 5.0) - 459.67
    }

    pub fn wind_speed(&self) -> f32 {
        self.wind.speed
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
