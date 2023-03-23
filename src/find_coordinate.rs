use std::collections::HashMap;

use serde_derive::Deserialize;

use crate::request::Request;

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct FindCoordinates {
    pub countryCode: String,
    pub country: String,
    pub region: String,
    pub city: String,
    pub latitude: f32,
    pub longitude: f32,
}

#[allow(dead_code)]
impl FindCoordinates {
    pub fn new(lat: &str, long: &str) -> Self {
        let request = Request::new(
            "https://namaz-vakti.vercel.app/api/place",
            None,
            Some(HashMap::from([
                ("lat".to_string(), lat.to_string()),
                ("lng".to_string(), long.to_string()),
            ])),
            None,
        );

        // deserialize the response
        let response = request.get().unwrap().text();

        let coordinates: FindCoordinates = serde_json::from_str(&response.unwrap()).unwrap();

        coordinates
    }

    pub fn get_coordinates(&self) -> &FindCoordinates {
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
