use std::collections::HashMap;

use serde_derive::Deserialize;

use crate::request::Request;

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Coordinates {
    pub country: String,
    pub countryCode: String,
    pub city: String,
    pub region: String,
    pub latitude: f32,
    pub longitude: f32,
}

#[allow(dead_code)]
impl Coordinates {
    pub fn new(country: &str, city: &str, region: &str) -> Self {
        let request = Request::new(
            "https://namaz-vakti.vercel.app/api/coordinates",
            None,
            Some(HashMap::from([
                ("country".to_string(), country.to_string()),
                ("city".to_string(), city.to_string()),
                ("region".to_string(), region.to_string()),
            ])),
            None,
        );

        // deserialize the response
        let response = request.get().unwrap().text();

        let coordinates: Coordinates = serde_json::from_str(&response.unwrap()).unwrap();

        coordinates
    }

    pub fn get_coordinates(&self) -> &Coordinates {
        &self
    }

    pub fn get_country(&self) -> &str {
        &self.country
    }

    pub fn get_country_code(&self) -> &str {
        &self.countryCode
    }

    pub fn get_city(&self) -> &str {
        &self.city
    }

    pub fn get_region(&self) -> &str {
        &self.region
    }

    pub fn get_latitude(&self) -> &f32 {
        &self.latitude
    }

    pub fn get_longitude(&self) -> &f32 {
        &self.longitude
    }
}
