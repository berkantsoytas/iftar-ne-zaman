use serde_derive::Deserialize;
use std::collections::HashMap;

use crate::request::Request;

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Place {
    country: String,
    countryCode: String,
    city: String,
    region: String,
    latitude: f32,
    longitude: f32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct TimeFromPlace {
    place: Place,
    times: HashMap<String, Vec<String>>,
}

#[allow(dead_code)]
impl TimeFromPlace {
    pub fn new(
        country: &str,
        region: &str,
        city: &str,
        date: &str,
        days: &str,
        timezone: &str,
    ) -> Self {
        let mut params: HashMap<String, String> = HashMap::new();

        params.insert("country".to_string(), country.to_string());
        params.insert("region".to_string(), region.to_string());
        params.insert("city".to_string(), city.to_string());
        params.insert("date".to_string(), date.to_string());
        params.insert("days".to_string(), days.to_string());
        params.insert("timezoneOffset".to_string(), timezone.to_string());

        let request = Request::new(
            "https://namaz-vakti.vercel.app/api/timesFromPlace",
            None,
            Some(params),
            None,
        );

        // deserialize the response
        let response = request.get().unwrap().text();

        let time_from_coordinates: TimeFromPlace =
            serde_json::from_str(&response.unwrap()).unwrap();

        time_from_coordinates
    }

    pub fn get_time(&self, timezone: &str) -> Option<&Vec<String>> {
        self.times.get(timezone)
    }

    pub fn get_place(&self) -> &Place {
        &self.place
    }

    pub fn get_times(&self) -> &HashMap<String, Vec<String>> {
        &self.times
    }

    pub fn get_country_code(&self) -> &str {
        &self.place.countryCode
    }

    pub fn get_country(&self) -> &str {
        &self.place.country
    }

    pub fn get_region(&self) -> &str {
        &self.place.region
    }

    pub fn get_city(&self) -> &str {
        &self.place.city
    }

    pub fn get_latitude(&self) -> f32 {
        self.place.latitude
    }

    pub fn get_longitude(&self) -> f32 {
        self.place.longitude
    }

    pub fn get_timezones(&self) -> Vec<&str> {
        let mut timezones: Vec<&str> = Vec::new();

        for (timezone, _) in &self.times {
            timezones.push(timezone);
        }

        timezones
    }
}
