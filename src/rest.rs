use error::*;
use model::*;
use reqwest::Client;
use serde_json as json;

pub fn query<S: AsRef<str>>(query: S) -> Result<Weather> {
    use API_KEY;
    use std::io::Read;
    
    let url = ApiUrl::new(API_KEY, query.as_ref());
    let response = {
        let mut response = Client::new().get(&url.into_url()).send()?;
        let mut buf = String::new();
        response.read_to_string(&mut buf).expect("Received invalid data");
        buf
    };

    json::from_str(&response).map_err(|e| {
        match json::from_str::<ApiError>(&response) {
            Ok(e) => Error::api(e),
            _ => Error::serialization(e),
        }
    })
}

#[derive(Debug)]
struct ApiUrl<'a> {
    key: &'a str,
    query: &'a str,
}

impl<'a> ApiUrl<'a> {
    fn new(key: &'a str, query: &'a str) -> Self {
        Self { key, query }
    }

    fn into_url(self) -> String {
        let Self { key, query } = self;
        let mut buf = String::new();
        
        buf.push_str("http://api.openweathermap.org/data/2.5/weather?q=");
        buf.push_str(query);
        buf.push_str("&APPID=");
        buf.push_str(key);

        buf
    }
}
