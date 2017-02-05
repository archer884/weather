use hyper::Client;
use model::Weather;
use serde_json as json;

pub type Result<T> = ::std::result::Result<T, String>;

pub struct WeatherClient(Client);

impl WeatherClient {
    pub fn new() -> WeatherClient {
        WeatherClient(Client::new())
    }

    pub fn query<T: AsRef<str>>(&self, q: T) -> Result<Weather> {
        use std::io::Read;

        let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&APPID={}",
                          q.as_ref(),
                          &dotenv!("API_KEY"));

        match self.0.get(&url).send() {
            Err(_) => Err("server said no".into()),
            Ok(mut response) => {
                let mut buf = String::new();
                response.read_to_string(&mut buf);
                Ok(json::from_str(&buf).expect(&format!("bad response from server: {}", buf)))
            }
        }
    }
}
