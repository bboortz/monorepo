/*
*/

#[derive(Debug)]
pub enum ErrorType {
    // Errors from external libraries...
    Io(std::io::Error),
    // Errors raised by us...
    Regular(ErrorKind),
    Custom(CustomError),
}

impl From<std::io::Error> for ErrorType {
    fn from(err: std::io::Error) -> ErrorType {
        ErrorType::Io(err)
    }
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ErrorType::Io(ref err) => {
                write!(f, "An IO error occurred:\n    type:     {}", err)
            }
            ErrorType::Regular(ref err) => {
                write!(
                    f,
                    "A regular program error occurred:\n    type:        {}",
                    err.as_str()
                )
            }
            ErrorType::Custom(ref err) => write!(f, "A custom error occurred: {}", err),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    FileNotFound,
    InsufficientPermissions,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ErrorKind::FileNotFound => write!(f, "File not nound"),
            ErrorKind::InsufficientPermissions => write!(f, "Insufficient Permissions"),
        }
    }
}

impl ErrorKind {
    fn as_str(&self) -> &str {
        match *self {
            ErrorKind::FileNotFound => "file not found",
            ErrorKind::InsufficientPermissions => "unsufficient permissions",
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub affected: String,
    pub suggestion: String,
}

impl Error {
    pub fn set_affected(&mut self, affected: String) {
        self.affected = affected;
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}\n    affected:    {}\n    suggestion:  {}",
            self.error_type, self.affected, self.suggestion,
        )
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        let error_affected = String::from("unknown1");
        let error_suggestion = String::from("-");
        Error {
            error_type: ErrorType::Io(err),
            affected: error_affected,
            suggestion: error_suggestion,
        }
    }
}

#[derive(Debug)]
pub struct CustomError {
    pub error_string: String,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ERROR: {}", self.error_string)
    }
}

#[cfg(test)]
mod tests {
    use crate::error;

    fn raise_err_trait_std_io_error() -> std::result::Result<usize, error::Error> {
        let error_affected = String::from("file test.file");
        let error_suggestion = String::from("testcase");
        let io_error = std::io::Error::new(std::io::ErrorKind::Other, "oh no!");
        let error_type = error::ErrorType::Io(io_error);
        let err = error::Error {
            error_type: error_type,
            affected: error_affected,
            suggestion: error_suggestion,
        };
        Err(err)
    }

    #[test]
    fn test_raise_err_trait_std_io_error() {
        match raise_err_trait_std_io_error() {
            Ok(_) => assert!(false, "Need to return an Error!"),
            Err(e) => {
                println!("{}", e);
                println!("{:?}", e);
                println!("{}", e.error_type);
                println!("{:?}", e.error_type);
                assert!(true);
            }
        }
    }

    fn raise_err_regular_filenotfound() -> Result<usize, error::Error> {
        let error_affected = String::from("file test.file");
        let error_suggestion = String::from("testcase");
        let error_type = error::ErrorType::Regular(error::ErrorKind::FileNotFound);
        let err = error::Error {
            error_type: error_type,
            affected: error_affected,
            suggestion: error_suggestion,
        };
        Err(err)
    }

    #[test]
    fn test_err_regular_filenotfound() {
        match raise_err_regular_filenotfound() {
            Ok(_) => assert!(false, "Need to return an Error!"),
            Err(e) => {
                println!("{}", e);
                println!("{:?}", e);
                println!("{}", e.error_type);
                println!("{:?}", e.error_type);
                assert!(true);
            }
        }
    }

    fn raise_err_regular_insufficientpermissions() -> Result<usize, error::Error> {
        let error_affected = String::from("file test.file");
        let error_suggestion = String::from("testcase");
        let error_type = error::ErrorType::Regular(error::ErrorKind::InsufficientPermissions);
        let err = error::Error {
            error_type: error_type,
            affected: error_affected,
            suggestion: error_suggestion,
        };
        Err(err)
    }

    #[test]
    fn test_err_regular_insufficientpermissions() {
        match raise_err_regular_insufficientpermissions() {
            Ok(_) => assert!(false, "Need to return an Error!"),
            Err(e) => {
                println!("{}", e);
                println!("{:?}", e);
                println!("{}", e.error_type);
                println!("{:?}", e.error_type);
                assert!(true);
            }
        }
    }

    fn raise_err_custom() -> Result<usize, error::Error> {
        let error_string = String::from("Line not found");
        let error_affected = String::from("file test.file");
        let error_suggestion = String::from("testcase");
        let custom_error = error::CustomError { error_string };
        let error_type = error::ErrorType::Custom(custom_error);
        let err = error::Error {
            error_type: error_type,
            affected: error_affected,
            suggestion: error_suggestion,
        };
        Err(err)
    }

    #[test]
    fn test_raise_err_custom() {
        match raise_err_custom() {
            Ok(_) => assert!(false, "Need to return an Error!"),
            Err(e) => {
                println!("{}", e);
                println!("{:?}", e);
                println!("{}", e.error_type);
                println!("{:?}", e.error_type);
                assert!(true);
            }
        }
    }

    #[test]
    fn test_error_from_std_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::Other, "oh no!");
        let _error = error::Error::from(io_error);
    }

    #[test]
    fn test_errortype_from_std_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::Other, "oh no!");
        let _error = error::ErrorType::from(io_error);
    }
}
