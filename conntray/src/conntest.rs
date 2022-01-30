use std::time;

use crate::error;
use crate::reqwest_client;

const RETRIES: u16 = 3;

#[derive(Debug)]
pub struct ConnTestResult {
    pub url: String,
    pub status: u16,
    pub elapsed: std::time::Duration,
    pub total_elapsed: std::time::Duration,
    pub retries: u16,
}

#[derive(Debug)]
pub struct ConnTestReport {
    pub number_tests: u32,
    pub number_errors: u16,
    pub number_retries: u16,
    pub elapsed_min: std::time::Duration,
    pub elapsed_max: std::time::Duration,
    pub elapsed_mean: std::time::Duration,
    pub total_elapsed: std::time::Duration,
}

impl ConnTestReport {
    pub fn new(number_tests: u32) -> Self {
        Self {
            number_tests,
            number_errors: 0,
            number_retries: 0,
            elapsed_min: std::time::Duration::from_millis(0),
            elapsed_max: std::time::Duration::from_millis(0),
            elapsed_mean: std::time::Duration::from_millis(0),
            total_elapsed: std::time::Duration::from_millis(0),
        }
    }

    pub fn add_result(&mut self, result: &ConnTestResult) {
        // number_retries
        self.number_retries += result.retries;

        // elapsed_min
        if self.elapsed_min == std::time::Duration::from_millis(0) {
            self.elapsed_min += result.total_elapsed;
        }
        if result.total_elapsed < self.elapsed_min {
            self.elapsed_min = result.total_elapsed;
        }

        // elapsed_max
        if result.total_elapsed > self.elapsed_max {
            self.elapsed_max = result.total_elapsed;
        }

        // total_elapsed
        self.total_elapsed += result.total_elapsed;

        // elapsed_mean
        self.elapsed_mean = self.total_elapsed / self.number_tests;
    }

    pub fn add_error(&mut self, _result: &error::Error) {
        self.number_errors += 1;
        self.number_retries += RETRIES;
    }
}

fn run_single_url(url: &str) -> Result<ConnTestResult, error::Error> {
    info!("testing {} ...", url);
    let error_string = String::from("Standard Error");
    let error_reason = String::from("timed out");
    let error_affected = String::from(url);
    let error_suggestion = String::from("verify your connection");
    let custom_error = error::CustomError { error_string };
    let error_type = error::ErrorType::Custom(custom_error);
    let mut err = error::Error {
        error_type,
        reason: error_reason,
        affected: error_affected,
        suggestion: error_suggestion,
    };

    let c = reqwest_client::ReqwestClient::new();
    let now = time::Instant::now();

    for i in 0..RETRIES {
        match c.test_url_async(url) {
            Ok(mut res) => match res.status {
                200 => {
                    res.retries = i;
                    res.total_elapsed = now.elapsed();
                    return Ok(res);
                }
                _ => {
                    warn!("HTTP Code != 200: {}", res.status);
                }
            },
            Err(e) => {
                warn!("{}", e);
                err = e;
            }
        }
    }

    Err(err)
}

pub fn run() -> Result<(), error::Error> {
    let url1 = String::from("https://httpbin.org/ip");
    let url2 = String::from("https://ifconfig.co/json");
    let url3 = String::from("https://api.ipify.org?format=json");
    let url_arr = [url1, url2, url3];

    info!("run connection test with {} urls ...", url_arr.len());

    let mut report = ConnTestReport::new(u32::try_from(url_arr.len()).unwrap());
    for url in &url_arr {
        match run_single_url(url) {
            Ok(res) => {
                report.add_result(&res);
                debug!("<-- {:#?}", res);
            }
            Err(e) => {
                report.add_error(&e);
                error!("{}", e);
            }
        }
    }
    info!("REPORT: {:#?}", report);

    Ok(())
}
