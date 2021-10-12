use std::io::Error;

#[derive(Debug)]
pub enum ErrorType {
    // Errors from external libraries...
    Io(Error),
    // Errors raised by us...
    Regular(ErrorKind),
    Custom(String),
}

impl From<std::io::Error> for ErrorType {
    fn from(err: std::io::Error) -> ErrorType {
        ErrorType::Io(err)
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

/*
impl std::error::Error for ErrorType {
    fn description(&self) -> &str {
        match *self {
            ErrorType::Io(ref err) => err.description(),
            ErrorType::Git(ref err) => err.description(),
            ErrorType::Regular(ref err) => err.as_str(),
            ErrorType::Custom(ref err) => err,
        }
    }
}
*/

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ErrorType::Io(ref err) => err.fmt(f),
            ErrorType::Regular(ref err) => write!(f, "A regular error occurred: {}", err.as_str()),
            ErrorType::Custom(ref err) => write!(f, "A custom error occurred: {}", err),
        }
    }
}

////

#[cfg(test)]
mod tests {
    use crate::error;
    /*
     */
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
