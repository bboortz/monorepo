use crate::error;
use std::process;

pub struct Logga {}

impl Logga {
    /*
    pub fn new() -> Logga {
        Logga {}
    }
    */

    #[allow(dead_code)]
    pub fn info(&self, str: &str) {
        println!("INFO: {}", str);
    }

    pub fn error(&self, str: &error::Error) {
        eprintln!("ERROR: {}", str);
    }

    pub fn panic(&self, str: &error::Error) {
        self.error(str);
        process::exit(1);
    }

    // usage:    LOGGA.print_type_of(&opt);
    #[allow(dead_code)]
    pub fn print_type_of<T>(&self, _: &T) {
        println!("{}", std::any::type_name::<T>())
    }
}

/*
impl Default for Logga {
    fn default() -> Self {
        Self::new()
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    static LOGGA: Logga = Logga {};

    #[test]
    fn test_new() {
        let _logga = Logga {};
    }

    #[test]
    fn test_info() {
        LOGGA.info("test info");
    }

    #[test]
    fn test_print_type_of() {
        LOGGA.print_type_of(&"foobar");
    }

    #[test]
    fn test_error() {
        let error_affected = String::from("file test.file");
        let error_suggestion = String::from("testcase");
        let error_type = error::ErrorType::Regular(error::ErrorKind::FileNotFound);
        let err = error::Error {
            error_type: error_type,
            affected: error_affected,
            suggestion: error_suggestion,
        };
        LOGGA.error(&err);
    }
}
