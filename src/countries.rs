use serde_derive::Deserialize;

use crate::request::Request;

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Countries(Vec<Country>);

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Country {
    pub code: String,
    pub name: String,
}

#[allow(dead_code)]
impl Countries {
    pub fn new() -> Self {
        let request = Request::new(
            "https://namaz-vakti.vercel.app/api/countries",
            None,
            None,
            None,
        );

        // deserialize the response
        let response = request.get().unwrap().text();

        let countries: Countries = serde_json::from_str(&response.unwrap()).unwrap();

        countries
    }

    pub fn get_country(&self, code: &str) -> Option<&Country> {
        self.0.iter().find(|country| country.code == code)
    }

    pub fn get_countries(&self) -> &Vec<Country> {
        &self.0
    }
}
