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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_as_byte_vec() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x00, 0x10, 0x20, 0x31, 0x32, 0x33,
            0x20, 0x11,
        ];
        let path = std::path::PathBuf::from(r"./samples/handcrafted_ident");
        let val = get_file_as_byte_vec(&path).unwrap();

        assert_eq!(data, val);
    }

    #[test]
    fn test_parse_file() {
        use deku::DekuContainerWrite;

        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x00, 0x10, 0x20, 0x31, 0x32, 0x33,
            0x20, 0x11, 0x00, 0x00, 0x00, 0x00,
        ];
        let path = std::path::PathBuf::from(r"./samples/handcrafted_ident");
        let val = parse_file(&path).unwrap();

        let data_out = val.to_bytes().unwrap();
        assert_eq!(data, data_out);
    }

    #[test]
    fn test_parse_nonexisting_file() {
        let path = std::path::PathBuf::from(r"/path/to/nonexisting/file");
        let val = parse_file(&path);

        match val {
            Ok(_) => assert!(false, "Need to return an Error!"),
            Err(e) => {
                println!("{}", e);
                println!("{:?}", e);
                println!("{}", e.error_type);
                println!("{:?}", e.error_type);
                assert!(true);
            }
        };
    }

    #[test]
    fn test_parse_nopermissions_file() {
        let path = std::path::PathBuf::from(r"/proc/kcore");
        let val = parse_file(&path);

        match val {
            Ok(_) => assert!(false, "Need to return an Error!"),
            Err(e) => {
                println!("{}", e);
                println!("{:?}", e);
                println!("{}", e.error_type);
                println!("{:?}", e.error_type);
                assert!(true);
            }
        };
    }
}
