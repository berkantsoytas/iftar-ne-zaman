use std::collections::HashMap;

use reqwest::blocking::{Client, Response};

#[allow(dead_code)]
pub struct Request {
    url: String,
    headers: Option<HashMap<String, String>>,
    params: Option<HashMap<String, String>>,
    body: Option<String>,
}

#[allow(dead_code)]
impl Request {
    pub fn new(
        url: &str,
        headers: Option<HashMap<String, String>>,
        params: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Self {
        Self {
            url: url.to_string(),
            headers,
            params,
            body,
        }
    }

    pub fn get(&self) -> Result<Response, reqwest::Error> {
        let client = Client::new();

        let mut req_builder = client.get(&self.url);

        if let Some(headers) = &self.headers {
            for (key, value) in headers.iter() {
                req_builder = req_builder.header(key, value);
            }
        }

        if let Some(params) = &self.params {
            for (key, value) in params.iter() {
                req_builder = req_builder.query(&[(key, value)]);
            }
        }

        req_builder.send()
    }

    pub fn post(&self) -> Result<Response, reqwest::Error> {
        let client = Client::new();

        let mut req_builder = client.post(&self.url);

        if let Some(headers) = &self.headers {
            for (key, value) in headers.iter() {
                req_builder = req_builder.header(key, value);
            }
        }

        if let Some(params) = &self.params {
            for (key, value) in params.iter() {
                req_builder = req_builder.query(&[(key, value)]);
            }
        }

        if let Some(body) = &self.body {
            req_builder = req_builder.body(body.to_owned());
        }

        req_builder.send()
    }

    pub fn put(&self) -> Result<Response, reqwest::Error> {
        let client = Client::new();
        let mut req_builder = client.put(&self.url);

        if let Some(headers) = &self.headers {
            for (key, value) in headers.iter() {
                req_builder = req_builder.header(key, value);
            }
        }

        if let Some(params) = &self.params {
            for (key, value) in params.iter() {
                req_builder = req_builder.query(&[(key, value)]);
            }
        }

        if let Some(body) = &self.body {
            req_builder = req_builder.body(body.to_owned());
        }

        req_builder.send()
    }
}
