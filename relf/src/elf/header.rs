use crate::elf::header_ident;
use crate::elf::header_machine;
use crate::elf::header_type;
use crate::elf::header_version;
use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku()]
pub struct FileHeader {
    pub ident: header_ident::HeaderIdent,
    pub r#type: header_type::HeaderType,
    pub machine: header_machine::HeaderMachine,
    pub version: header_version::HeaderVersion,
}

impl fmt::Display for FileHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "----------------   HEADER   ---\n{}\n{}\n{}\n{}",
            self.ident, self.r#type, self.machine, self.version
        )
    }
}

impl Default for FileHeader {
    fn default() -> Self {
        FileHeader {
            ident: header_ident::HeaderIdent::new(),
            r#type: header_type::HeaderType::new(),
            machine: header_machine::HeaderMachine::new(),
            version: header_version::HeaderVersion::new(),
        }
    }
}

impl FileHeader {
    pub fn new() -> Self {
        FileHeader {
            ident: header_ident::HeaderIdent::new(),
            r#type: header_type::HeaderType::new(),
            machine: header_machine::HeaderMachine::new(),
            version: header_version::HeaderVersion::new(),
        }
    }

    fn reader(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, Vec<u8>), DekuError> {
        debug!("rest: {:?}", rest);
        debug!("len rest: {}", rest.len());

        // slice off length of options
        let index = 4 * 8;

        // Check split_at precondition
        if index > rest.len() {
            return Err(DekuError::Parse(format!(
                "Not enough data to read HeaderIdent. Bits expected: {}, Bits given: {}",
                index,
                rest.len()
            )));
        }

        // read data
        let mut ret = vec![];
        let (mut u8_rest, rest) = rest.split_at(index);
        while !u8_rest.is_empty() {
            let (u8_rest_new, u8_val) = u8::read(u8_rest, deku::ctx::Endian::Little)?;
            ret.push(u8_val);
            u8_rest = u8_rest_new;
        }
        Ok((rest, ret))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elf::header_ident;
    use crate::elf::header_ident_abiversion;
    use crate::elf::header_ident_class;
    use crate::elf::header_ident_endian;
    use crate::elf::header_ident_magic;
    use crate::elf::header_ident_osabi;
    use crate::elf::header_ident_padding;
    use crate::elf::header_ident_version;

    #[test]
    fn test_from_bytes() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00, 0x3E, 0x00, 0x00, 0x01, 0x00, 0x00,
        ];
        let (rest, val) = FileHeader::from_bytes((&data, 0)).unwrap();
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
        let expected = FileHeader {
            ident: ident,
            r#type: header_type,
            machine: header_machine,
            version: header_version,
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

        let err = FileHeader::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_incomplete() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00, 0x3E, 0x00, 0x00, 0x01, 0x00,
        ];

        let err = FileHeader::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_invalid() {
        let data: Vec<u8> = vec![
            0x77, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00, 0x3E, 0x00, 0x00, 0x01, 0x00, 0x00,
        ];

        let err = FileHeader::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_try_from() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x00, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04,
        ];

        let val = FileHeader::try_from(data.as_ref()).unwrap();
        println!("value: {:?}", val);

        let expected = FileHeader::new();
        assert_eq!(expected, val);

        let value: Vec<u8> = val.try_into().unwrap();
        assert_eq!(data, value);
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bytes() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00, 0x3E, 0x00,
        ];
        let val = parse_bytes(&data).unwrap();

        let header_ident_magic = HeaderIdentMagic {
            ei_mag0: 0x7F,
            ei_mag1: 0x45,
            ei_mag2: 0x4C,
            ei_mag3: 0x46,
        };
        let header_ident_class = HeaderIdentClass::Bit32;
        let header_ident_endian = HeaderIdentEndian::Little;
        let header_ident_version = HeaderIdentVersion { ei_version: 0x01 };
        let header_ident_osabi = HeaderIdentOsAbi::Linux;
        let header_ident_abiversion = HeaderIdentAbiVersion {
            ei_abiversion: 0x01,
        };
        let header_ident_padding = HeaderIdentPadding {
            ei_pad: vec![0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17],
        };
        let ident = HeaderIdent {
            magic: header_ident_magic,
            class: header_ident_class,
            endian: header_ident_endian,
            version: header_ident_version,
            osabi: header_ident_osabi,
            abiversion: header_ident_abiversion,
            padding: header_ident_padding,
        };
        let header_type = HeaderType::Dyn;
        let header_machine = HeaderMachine::X86_64;
        let header_version = HeaderVersion {
            ei_version: vec![0x00, 0x01, 0x00, 0x00],
        };
        let header = FileHeader {
            ident: ident,
            r#type: header_type,
            machine: header_machine,
            //            version: header_version,
        };
        let expected = ElfFile {
            file_header: header,
        };
        println!("{}", &val);
        assert_eq!(expected, val);

        let data_out = val.to_bytes().unwrap();
        assert_eq!(data, data_out);
    }

    #[test]
    fn test_parse_bytes_header_ident_direct() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x06, 0x07,
        ];
        let val = parse_bytes_header_ident(&data).unwrap();

        let header_ident_magic = HeaderIdentMagic {
            ei_mag0: 0x7F,
            ei_mag1: 0x45,
            ei_mag2: 0x4C,
            ei_mag3: 0x46,
        };
        let header_ident_class = HeaderIdentClass::Bit32;
        let header_ident_endian = HeaderIdentEndian::Little;
        let header_ident_version = HeaderIdentVersion { ei_version: 0x01 };
        let header_ident_osabi = HeaderIdentOsAbi::Linux;
        let header_ident_abiversion = HeaderIdentAbiVersion {
            ei_abiversion: 0x01,
        };
        let header_ident_padding = HeaderIdentPadding {
            ei_pad: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
        };
        let ident = HeaderIdent {
            magic: header_ident_magic,
            class: header_ident_class,
            endian: header_ident_endian,
            version: header_ident_version,
            osabi: header_ident_osabi,
            abiversion: header_ident_abiversion,
            padding: header_ident_padding,
        };

        assert_eq!(ident, val);

        let data_out = val.to_bytes().unwrap();
        assert_eq!(data, data_out);
    }

    #[test]
    fn test_parse_bytes_header_ident_only() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x06, 0x07,
        ];
        let val = parse_bytes(&data).unwrap();

        let header_ident_magic = HeaderIdentMagic {
            ei_mag0: 0x7F,
            ei_mag1: 0x45,
            ei_mag2: 0x4C,
            ei_mag3: 0x46,
        };
        let header_ident_class = HeaderIdentClass::Bit32;
        let header_ident_endian = HeaderIdentEndian::Little;
        let header_ident_version = HeaderIdentVersion { ei_version: 0x01 };
        let header_ident_osabi = HeaderIdentOsAbi::Linux;
        let header_ident_abiversion = HeaderIdentAbiVersion {
            ei_abiversion: 0x01,
        };
        let header_ident_padding = HeaderIdentPadding {
            ei_pad: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
        };
        let ident = HeaderIdent {
            magic: header_ident_magic,
            class: header_ident_class,
            endian: header_ident_endian,
            version: header_ident_version,
            osabi: header_ident_osabi,
            abiversion: header_ident_abiversion,
            padding: header_ident_padding,
        };
        let header_type = HeaderType::None;
        let header_machine = HeaderMachine::None;
        let header_version = HeaderVersion {
            ei_version: vec![0x01, 0x01, 0x01, 0x01],
        };
        let header = FileHeader {
            ident: ident,
            r#type: header_type,
            machine: header_machine,
            //            version: header_version,
        };
        let expected = ElfFile {
            file_header: header,
        };
        assert_eq!(expected, val);
    }

    #[test]
    fn test_parse_bytes_invalid_magic() {
        let data: Vec<u8> = vec![0x7F, 0x45, 0x4C, 0x45];
        let val = parse_bytes(&data);

        match val {
            Ok(_) => assert!(false, "Need to return an Error!"),
            Err(e) => {
                println!("{}", e.error_type);
                println!("{:?}", e.error_type);
                match e.error_type {
                    error::ErrorType::Deku(d) => match d {
                        deku::error::DekuError::Assertion(_s) => {
                            assert!(true);
                        }
                        _ => {
                            assert!(false);
                        }
                    },
                    _ => {
                        assert!(false);
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_bytes_incomplete_endian() {
        let data: Vec<u8> = vec![0x7F, 0x45, 0x4C, 0x46, 0x01];
        let val = parse_bytes(&data);

        match val {
            Ok(_) => assert!(false, "Need to return an Error!"),
            Err(e) => {
                println!("{}", e.error_type);
                println!("{:?}", e.error_type);
                match e.error_type {
                    error::ErrorType::Deku(d) => match d {
                        deku::error::DekuError::Incomplete(_s) => {
                            assert!(true);
                        }
                        _ => {
                            assert!(false);
                        }
                    },
                    _ => {
                        assert!(false);
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_bytes_incomplete_version() {
        let data: Vec<u8> = vec![0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01];
        let val = parse_bytes(&data);

        match val {
            Ok(_) => assert!(false, "Need to return an Error!"),
            Err(e) => {
                println!("{}", e.error_type);
                println!("{:?}", e.error_type);
                match e.error_type {
                    error::ErrorType::Deku(d) => match d {
                        deku::error::DekuError::Incomplete(_s) => {
                            assert!(true);
                        }
                        _ => {
                            assert!(false);
                        }
                    },
                    _ => {
                        assert!(false);
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_bytes_incomplete_osabi() {
        let data: Vec<u8> = vec![0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01];
        let val = parse_bytes(&data);

        match val {
            Ok(_) => assert!(false, "Need to return an Error!"),
            Err(e) => {
                println!("{}", e.error_type);
                println!("{:?}", e.error_type);
                match e.error_type {
                    error::ErrorType::Deku(d) => match d {
                        deku::error::DekuError::Incomplete(_s) => {
                            assert!(true);
                        }
                        _ => {
                            assert!(false);
                        }
                    },
                    _ => {
                        assert!(false);
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_bytes_incomplete_abiversion() {
        let data: Vec<u8> = vec![0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03];
        let val = parse_bytes(&data);

        match val {
            Ok(_) => assert!(false, "Need to return an Error!"),
            Err(e) => {
                println!("{}", e.error_type);
                println!("{:?}", e.error_type);
                match e.error_type {
                    error::ErrorType::Deku(d) => match d {
                        deku::error::DekuError::Incomplete(_s) => {
                            assert!(true);
                        }
                        _ => {
                            assert!(false);
                        }
                    },
                    _ => {
                        assert!(false);
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_bytes_incomplete_padding() {
        let data: Vec<u8> = vec![0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01];
        let val = FileHeader::from_bytes((&data, 0));

        match val {
            Ok(_) => assert!(false, "Need to return an Error!"),
            Err(e) => {
                println!("{}", e.error_type);
                println!("{:?}", e.error_type);
                match e.error_type {
                    error::ErrorType::Deku(d) => match d {
                        deku::error::DekuError::Incomplete(_s) => {
                            assert!(true);
                        }
                        _ => {
                            assert!(false);
                        }
                    },
                    _ => {
                        assert!(false);
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_bytes_incomplete_type() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17,
        ];
        println!("len orig: {}", data.len());
        let _err = FileHeader::from_bytes((&data, 0)).unwrap_err();
        /*
        let val_type = val.r#type;
        let expected = header_type::HeaderType::None;
        assert_eq!(expected, val_type);
        */
    }

    */
/*
    #[test]
    fn test_parse_header_invalid_machine() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00, 0x00, 0x00,
        ];
        println!("len orig: {}", data.len());
        let (_rest, val) = FileHeader::from_bytes((&data, 0)).unwrap();

        let header_ident_magic = header_ident::HeaderIdentMagic {
            ei_mag0: 0x7F,
            ei_mag1: 0x45,
            ei_mag2: 0x4C,
            ei_mag3: 0x46,
        };
        let header_ident_class = header_ident::HeaderIdentClass::Bit32;
        let header_ident_endian = header_ident::HeaderIdentEndian::Little;
        let header_ident_version = header_ident::HeaderIdentVersion { ei_version: 0x01 };
        let header_ident_osabi = header_ident::HeaderIdentOsAbi::Linux;
        let header_ident_abiversion = header_ident::HeaderIdentAbiVersion {
            ei_abiversion: 0x01,
        };
        let header_ident_padding = header_ident::HeaderIdentPadding {
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
        let header_machine = header_machine::HeaderMachine::None;
        let header_version = header_version::HeaderVersion {
            ei_version: vec![0x00, 0x01, 0x00, 0x00],
        };
        let expected = FileHeader {
            ident: ident,
            r#type: header_type,
            machine: header_machine,
            //            version: header_version,
        };
        println!("{}", &val);
        assert_eq!(expected, val);

        let data_out = val.to_bytes().unwrap();
        assert_eq!(data, data_out);
    }

    #[test]
    fn test_parse_bytes_incomplete_machine() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00,
        ];
        let _err = FileHeader::from_bytes((&data, 0)).unwrap_err();
        /*
        let val_type = val.r#type;
        let val_machine = val.machine;
        let expected_type = header_type::HeaderType::Dyn;
        let expected_machine = header_machine::HeaderMachine::None;
        assert_eq!(expected_type, val_type);
        assert_eq!(expected_machine, val_machine);
        */
    }

    #[test]
    fn test_parse_bytes_machine_direct() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00, 0x3E, 0x00,
        ];
        match FileHeader::from_bytes((&data, 0)) {
            Ok((rest, val)) => {
                println!("{:?}", rest);
                println!("{}", val);
                let val_type = val.r#type;
                let val_machine = val.machine;
                let expected_type = header_type::HeaderType::Dyn;
                let expected_machine = header_machine::HeaderMachine::X86_64;
                assert_eq!(expected_type, val_type);
                assert_eq!(expected_machine, val_machine);
            }
            Err(err) => {
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
                println!("{:?}", err);
                error!("{}", err);
                warn!("Incomplete ELF file. Trying to parse elf header ident ...");
                assert!(false);
            }
        }
    }
}
    */
