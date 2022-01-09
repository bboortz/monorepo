pub mod file;
pub mod header;
pub mod header_ident;
pub mod header_ident_abiversion;
pub mod header_ident_class;
pub mod header_ident_endian;
pub mod header_ident_magic;
pub mod header_ident_osabi;
pub mod header_ident_padding;
pub mod header_ident_version;
pub mod header_machine;
pub mod header_type;
pub mod header_version;
use crate::error;
use deku::prelude::*;

pub fn parse_bytes(data: &[u8]) -> Result<file::ElfFile, error::Error> {
    debug!("len: {}", data.len());
    match file::ElfFile::from_bytes((data, 0)) {
        Ok((_rest, val)) => Ok(val),
        Err(deku_err) => {
            /*
            let ident = match parse_bytes_header_ident(data) {
                Ok(val) => val,
                Err(err) => {
                    Err(err)
                  /*
                    let error_affected = String::from("unknown");
                    let error_suggestion = String::from("verify the file format");
                    let error_type = error::ErrorType::Deku(deku_err);
                    let err = error::Error {
                        error_type,
                        affected: error_affected,
                        suggestion: error_suggestion,
                    };
                    Err(err)
                  */
                }
            };
            */
            error!("{}", deku_err);
            warn!("Incomplete ELF file. Trying to parse elf header ident ...");
            let ident = header_ident::parse_bytes_header_ident(data)?;

            // Big TODO !
            // 1. move the parsing to dedicated functions
            // 2. decide if we let deku build the struct or if we are building it on our own
            let mut header_version = header_version::HeaderVersion::new();
            let mut header_machine: header_machine::HeaderMachine =
                header_machine::HeaderMachine::None;
            let mut header_type: header_type::HeaderType = header_type::HeaderType::None;
            if data.len() >= 24 {
                debug!("Bytes HeaderMachine - {:?}", &data[18..][..2]);
                header_version =
                    match header_version::HeaderVersion::from_bytes((&data[20..][..2], 0)) {
                        Ok((_rest_type, val_version)) => {
                            // header_type = val_type;
                            val_version
                        }
                        Err(type_err) => {
                            println!("{}", type_err);
                            error!("{}", type_err);
                            header_version::HeaderVersion::new()
                        }
                    };
            }
            if data.len() >= 20 {
                debug!("Bytes HeaderMachine - {:?}", &data[18..][..2]);
                // header_type = match header_type::HeaderType::from_bytes((&data[16..][..2], 0)) {
                header_machine =
                    match header_machine::HeaderMachine::from_bytes((&data[18..][..2], 0)) {
                        Ok((_rest_type, val_type)) => {
                            // header_type = val_type;
                            val_type
                        }
                        Err(type_err) => {
                            println!("{}", type_err);
                            error!("{}", type_err);
                            header_machine::HeaderMachine::None
                        }
                    };
            }
            if data.len() >= 18 {
                println!("Bytes HeaderType - {:?}", &data[16..][..2]);
                header_type = match header_type::HeaderType::from_bytes((&data[16..][..2], 0)) {
                    Ok((_rest_type, val_type)) => {
                        // header_type = val_type;
                        val_type
                    }
                    Err(type_err) => {
                        println!("{}", type_err);
                        error!("{}", type_err);
                        header_type::HeaderType::None
                    }
                };
            }

            let header = header::FileHeader {
                ident,
                r#type: header_type,
                machine: header_machine,
                version: header_version,
            };
            let elffile = file::ElfFile {
                file_header: header,
            };
            Ok(elffile)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bytes() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00, 0x3E, 0x00, 0x00, 0x01, 0x00, 0x00,
        ];
        let (rest, val) = file::ElfFile::from_bytes((&data, 0)).unwrap();
        println!("rest: {:?}", rest);
        println!("value: {:?}", val);

        let header_ident_magic = header_ident_magic::HeaderIdentMagic {
            ei_mag: vec![0x7F, 0x45, 0x4C, 0x46],
        };
        let header_ident_class = header_ident_class::HeaderIdentClass::Bit32;
        let header_ident_endian = header_ident_endian::HeaderIdentEndian::Little;
        let header_ident_version = header_ident_version::HeaderIdentVersion { ei_version: 0x01 };
        let header_ident_osabi = header_ident_osabi::HeaderIdentOsAbi::Linux;
        let header_ident_abiversion = header_ident_abiversion::HeaderIdentAbiVersion {
            ei_abiversion: 0x01,
        };
        let header_ident_padding = header_ident_padding::HeaderIdentPadding {
            ei_pad: vec![0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17],
        };
        let ident = header_ident::HeaderIdent {
            magic: header_ident_magic,
            class: header_ident_class,
            endian: header_ident_endian,
            version: header_ident_version,
            osabi: header_ident_osabi,
            abiversion: header_ident_abiversion,
            padding: header_ident_padding,
        };
        let header_type = header_type::HeaderType::Dyn;
        let header_machine = header_machine::HeaderMachine::X86_64;
        let header_version = header_version::HeaderVersion {
            ei_version: vec![0x00, 0x01, 0x00, 0x00],
        };
        let header = header::FileHeader {
            ident: ident,
            r#type: header_type,
            machine: header_machine,
            version: header_version,
        };
        let expected = file::ElfFile {
            file_header: header,
        };
        println!("{}", &val);
        assert_eq!(expected, val);

        let data_out = val.to_bytes().unwrap();
        assert_eq!(data, data_out);

        assert_eq!((vec![].as_ref(), 0 as usize), rest);
    }

    #[test]
    fn test_from_bytes_empty() {
        let data: Vec<u8> = vec![];

        let err = file::ElfFile::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_incomplete() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00, 0x3E, 0x00, 0x00, 0x01, 0x00,
        ];

        let err = file::ElfFile::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_invalid_header_magic() {
        let data: Vec<u8> = vec![
            0x7F, 0x00, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x01, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00, 0x3E, 0x00, 0x00, 0x01, 0x00, 0x00,
        ];

        let err = file::ElfFile::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_invalid_header_ident_class() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x11, 0x01, 0x01, 0x01, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00, 0x3E, 0x00, 0x00, 0x01, 0x00, 0x00,
        ];

        let err = file::ElfFile::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_invalid_header_ident_endian() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x11, 0x01, 0x01, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00, 0x3E, 0x00, 0x00, 0x01, 0x00, 0x00,
        ];

        let err = file::ElfFile::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_invalid_header_ident_version() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x11, 0x01, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00, 0x3E, 0x00, 0x00, 0x01, 0x00, 0x00,
        ];

        let err = file::ElfFile::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_invalid_header_ident_osabi() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x22, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00, 0x3E, 0x00, 0x00, 0x01, 0x00, 0x00,
        ];

        let err = file::ElfFile::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_invalid_header_type() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x01, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x05, 0x00, 0x3E, 0x00, 0x00, 0x01, 0x00, 0x00,
        ];

        let err = file::ElfFile::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_invalid_header_machine() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x01, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00, 0x99, 0x00, 0x00, 0x01, 0x00, 0x00,
        ];

        let err = file::ElfFile::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_try_from() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x00, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04,
        ];

        let val = file::ElfFile::try_from(data.as_ref()).unwrap();
        println!("value: {:?}", val);

        let expected = file::ElfFile::new();
        assert_eq!(expected, val);

        let value: Vec<u8> = val.try_into().unwrap();
        assert_eq!(data, value);
    }
}
