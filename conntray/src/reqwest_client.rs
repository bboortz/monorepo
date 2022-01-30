// use std::collections::HashMap;
use std::time;
use std::time::Duration;

use crate::error;
use crate::conntest;

const TIMEOUT: Duration = Duration::from_millis(2000);

pub struct ReqwestClient {
    client: reqwest::Client,
}

impl ReqwestClient {
    pub fn new() -> Self {
        let mut h = reqwest::header::HeaderMap::new();
        h.insert(
            "Accept",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        let client = reqwest::Client::builder()
            .default_headers(h)
            .timeout(TIMEOUT)
            .build()
            .unwrap();
        Self { client }
    }

    #[tokio::main]
    pub async fn test_url_async(&self, url: &str) -> Result<conntest::ConnTestResult, error::Error> {
        let req = self
            .client
            .get(url)
            .header("Accept", "application/json")
            .timeout(TIMEOUT);
        trace!("--> {:#?}", req);

        let now = time::Instant::now();

        return match req.send().await {
            Ok(val) => {
              trace!("<-- {:#?}", val);
              Ok(conntest::ConnTestResult {
                url: url.to_string(),
                status: val.status().as_u16(),
                elapsed: now.elapsed(),
                total_elapsed: now.elapsed(),
                retries: 0,
              })
            },
            Err(e) => {
              /*
              Ok(conntest::ConnTestResult {
                url: url.clone(),
                status: 0,
                elapsed: now.elapsed(),
                retries: 0,
              })
              */
                let mut error_reason = String::from("HTTP Error");
                if e.is_timeout() {
                  error_reason = String::from("Timeout Error");
                } else if e.is_connect() {
                  error_reason = String::from("Connection Error");
                }
                let error_affected = String::from(url);
                let error_suggestion = String::from("verify your connection.");
                let error_type = error::ErrorType::Reqwest(e);
                let err = error::Error {
                    error_type,
                    reason: error_reason,
                    affected: error_affected,
                    suggestion: error_suggestion,
                };

                Err(err)
            }
        };
        // println!("Elapsed {:?}", now.elapsed());

        // .json::<HashMap<String, String>>().await?;
        // thread::sleep(timeout);
        // println!("<-- {:#?}", resp.status());
    }
}
