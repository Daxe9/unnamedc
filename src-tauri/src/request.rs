use reqwest::{header::HeaderMap, StatusCode};
use std::{time::{Instant, Duration}};

#[derive(Debug)]
struct ResponseData {
    body: String,
    headers: HeaderMap,
    status_code: StatusCode,
    duration: u128
}
impl ResponseData {
    pub fn new(headers: &HeaderMap, status_code: StatusCode, duration: Duration, body: String) -> Self {
        ResponseData {
            duration: duration.as_micros(),
            headers: headers.clone(),
            status_code,
            body

        }
    }
}
enum APIResponse {
    Success(ResponseData),
    Failed
}


async fn get_request(url: &str) -> APIResponse {
    // TODO: share client for all requests
    let client = reqwest::Client::new();
    
    // measure api call time
    let start_timer = Instant::now();
    let raw_result = client.get(url).send().await;
    let duration = start_timer.elapsed();

    let result: APIResponse = match raw_result {
            Ok(raw_response) => {
                // initialize response_data
                let mut response_data = ResponseData::new(
                    raw_response.headers(),
                    raw_response.status(),
                    duration,
                    String::new()
                    
                        );

                // try to await body of api call
                let body = match raw_response.text().await {
                    Err(_) => return APIResponse::Failed,
                    Ok(v) => v 
                };
                
                // update body
                response_data.body = body;

                APIResponse::Success(response_data)

            },
            // if failed api call then return Failed
            Err(_) => APIResponse::Failed

        };

    result
}

