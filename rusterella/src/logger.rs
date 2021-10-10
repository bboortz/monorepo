use std::error::Error;
use std::process;

pub struct Logga {}

impl Logga {
    pub fn new() -> Logga {
        Logga {}
    }

    #[allow(dead_code)]
    fn info(&self, str: &str) {
        println!("INFO: {}", str);
    }

    pub fn error(&self, str: &Box<dyn Error>) {
        eprintln!("ERROR: {}", str);
    }

    pub fn panic(&self, str: &Box<dyn Error>) {
        self.error(str);
        process::exit(1);
    }
}

impl Default for Logga {
    fn default() -> Self {
        Self::new()
    }
}

/*
#[derive(Debug)]
struct TestError {
    details: String,
}

impl TestError {
    fn new(msg: &str) -> Self {
        TestError {
            details: msg.to_string(),
        }
    }
}
*/
/*
#[cfg(test)]
mod tests {
    use super::*;
    static LOGGA: Logga = Logga {};

    #[test]
    fn test_info() {
        LOGGA.info("test info");
    }

    #[test]
    fn test_error() {
        let contents = fs::read_to_string(config.filename)?;
        LOGGA.error(TestError::new("borked"));
    }
}
*/
