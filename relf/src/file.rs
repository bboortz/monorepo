use crate::elf;
use crate::error;
use std::path::Path;

pub fn get_file_as_byte_vec(filename: &Path) -> Result<Vec<u8>, error::Error> {
    match std::fs::read(&filename) {
        Ok(bytes) => Ok(bytes),
        Err(e) => {
            let error_affected = filename.to_str().unwrap_or("unknown file").to_string();
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                let err = error::Error {
                    error_type: error::ErrorType::Regular(
                        error::ErrorKind::InsufficientPermissions,
                    ),
                    affected: error_affected,
                    suggestion: String::from("Verify the permissions of the file."),
                };
                return Err(err);
            } else if e.kind() == std::io::ErrorKind::NotFound {
                let err = error::Error {
                    error_type: error::ErrorType::Regular(error::ErrorKind::FileNotFound),
                    affected: error_affected,
                    suggestion: String::from(
                        "Verify if the file exists and you have specified the filename correct.",
                    ),
                };
                return Err(err);
            } else {
            }

            panic!("{}", e);
        }
    }
}

pub fn parse_file(filename: &Path) -> Result<elf::ElfFile, error::Error> {
    let error_affected = filename.to_str().unwrap_or("unknown file").to_string();
    if !filename.exists() {
        let err = error::Error {
            error_type: error::ErrorType::Regular(error::ErrorKind::FileNotFound),
            affected: error_affected,
            suggestion: String::from(
                "Verify if the file exists and you have specified the filename correct.",
            ),
        };
        return Err(err);
    }

    let bytes = get_file_as_byte_vec(filename)?;
    match elf::parse_bytes(&bytes) {
        Ok(val) => Ok(val),
        Err(mut initial_err) => {
            initial_err.affected = error_affected;
            Err(initial_err)
        }
    }
}
