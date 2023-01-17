use reqwest::{header::HeaderMap, StatusCode};
use std::{time::{Instant, Duration}, collections::HashMap};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct ResponseData {
    body: String,
    headers: HashMap<String, String>,
    status_code: u16,
    duration: u64
}

impl ResponseData {
    pub fn new(tmp_headers: HeaderMap, status_code: StatusCode, duration: Duration, body: String) -> Self {
        let mut headers: HashMap<String, String> = HashMap::new(); 
        for (key, value) in tmp_headers.iter() {
            headers.insert(key.to_string(), value.to_str().unwrap().to_string());
        }
        let status_code = status_code.as_u16();
        let duration: u64 = duration.as_micros() as u64;
        Self {
            duration,
            headers,
            status_code,
            body
        }
    }
}

// NOTE: https://jonaskruckenberg.github.io/tauri-docs-wip/development/inter-process-communication.html#error-handling
#[derive(Debug, thiserror::Error)]
pub enum APIError {
    #[error("Generic Error")]
    GenericError(#[from] std::io::Error),
    #[error("Request error")]
    RequestError(#[from] reqwest::Error),
    #[error("JSON error")]
    ParseError(#[from] serde_json::Error),
}
// we must manually implement serde::Serialize
impl serde::Serialize for APIError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            serializer.serialize_str(self.to_string().as_ref())
        }
}

#[tauri::command]
pub async fn get_request(url: &str) -> Result<ResponseData, APIError> {
    // TODO: share client for all requests

    let client = reqwest::Client::new();

    // measure api call time
    let start_timer = Instant::now();
    // make get request
    let raw_result = client.get(url).send().await?;
    // end  the timer
    let duration = start_timer.elapsed();
    
    // get complementary data
    let headers = raw_result.headers().clone();
    let status = raw_result.status();
    
    // create the response data to send back to the frontend
    let response_data = ResponseData::new(
        headers,
        status,
        duration,
        raw_result.text().await?,
        );

    Ok(response_data)
}

#[tauri::command]
pub async fn post_request(url: &str, raw_body: &str) -> Result<ResponseData, APIError> {
    // TODO: add headers
    // parse the body
    let body: Value = serde_json::from_str(raw_body)?;

    // inizialize the client
    let client = reqwest::Client::new();
    
    // measure api call time
    let start_timer = Instant::now();
    // make the post request with given body
    let raw_result = match client.post(url).json(&body).send().await {
        Ok(v) => v,
        Err(err) => return Err(APIError::RequestError(err))
    };
    // end  the timer
    let duration = start_timer.elapsed();

    // get complementary data
    let headers = raw_result.headers().clone();
    let status = raw_result.status();
    
    // create the response data to send back to the frontend
    let response_data = ResponseData::new(
        headers,
        status,
        duration,
        raw_result.text().await?,
        );

    Ok(response_data)
}
