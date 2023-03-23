use std::collections::HashMap;

use serde_derive::Deserialize;

use crate::request::Request;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CitiesRegions(Vec<String>);

#[allow(dead_code)]
impl CitiesRegions {
    pub fn new(country: &str, city: &str) -> Self {
        let request = Request::new(
            "https://namaz-vakti.vercel.app/api/cities",
            None,
            Some(HashMap::from([
                ("country".to_string(), country.to_string()),
                ("region".to_string(), city.to_string()),
            ])),
            None,
        );

        // deserialize the response
        let response = request.get().unwrap().text();

        let regions: CitiesRegions = serde_json::from_str(&response.unwrap()).unwrap();

        regions
    }

    pub fn get_regions(&self) -> &Vec<String> {
        &self.0
    }

    // find the region
    pub fn find(&self, region: &str) -> Option<&String> {
        self.0.iter().find(|&r| r == region)
    }

    // find the region index
    pub fn find_index(&self, region: &str) -> Option<usize> {
        self.0.iter().position(|r| r == region)
    }

    // check if the region exists
    pub fn exists(&self, region: &str) -> bool {
        self.find(region).is_some()
    }

    // check if the region exists
    pub fn exists_index(&self, region: &str) -> bool {
        self.find_index(region).is_some()
    }

    // get the region count
    pub fn count(&self) -> usize {
        self.0.len()
    }

    // get the region count
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
