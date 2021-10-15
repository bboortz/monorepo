/*
*/

#[derive(Debug)]
pub enum ErrorType {
    // Errors from external libraries...
    Io(std::io::Error),
    // Errors raised by us...
    Regular(ErrorKind),
    Custom(String),
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
        let error_affected = String::from("unknown");
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

    fn raise_err_trait_std_io_error_new() -> std::result::Result<usize, error::Error> {
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

    fn raise_err_regular_new() -> Result<usize, error::Error> {
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

    fn raise_err_custom_new() -> Result<usize, error::Error> {
        let error_string = String::from("Line not found");
        let error_affected = String::from("file test.file");
        let error_suggestion = String::from("testcase");
        let error_type = error::ErrorType::Custom(error_string);
        let err = error::Error {
            error_type: error_type,
            affected: error_affected,
            suggestion: error_suggestion,
        };
        Err(err)
    }

    #[test]
    fn test_error_new() {
        raise_err_trait_std_io_error_new();
        raise_err_regular_new();
        raise_err_custom_new();
    }

    fn raise_err_trait_std_io_error() -> std::result::Result<usize, error::ErrorType> {
        let _f = std::fs::File::create("/file/not/found/foobar")?;
        Err(error::ErrorType::Regular(error::ErrorKind::FileNotFound))
    }

    fn raise_err_regular() -> Result<usize, error::ErrorType> {
        Err(error::ErrorType::Regular(error::ErrorKind::FileNotFound))
    }

    fn raise_err_custom() -> Result<usize, error::ErrorType> {
        let custom_string = String::from("Custom Error String");
        let custom_error = error::ErrorType::Custom(custom_string);
        Err(custom_error)
    }

    #[test]
    fn test_error_type() {
        raise_err_trait_std_io_error();
        raise_err_regular();
        raise_err_custom();
    }
}
