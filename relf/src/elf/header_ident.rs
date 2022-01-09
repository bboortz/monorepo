use crate::elf::header_ident_abiversion;
use crate::elf::header_ident_class;
use crate::elf::header_ident_endian;
use crate::elf::header_ident_magic;
use crate::elf::header_ident_osabi;
use crate::elf::header_ident_padding;
use crate::elf::header_ident_version;
use crate::error;
use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku()]
pub struct HeaderIdent {
    pub magic: header_ident_magic::HeaderIdentMagic,
    pub class: header_ident_class::HeaderIdentClass,
    pub endian: header_ident_endian::HeaderIdentEndian,
    pub version: header_ident_version::HeaderIdentVersion,
    pub osabi: header_ident_osabi::HeaderIdentOsAbi,
    pub abiversion: header_ident_abiversion::HeaderIdentAbiVersion,
    pub padding: header_ident_padding::HeaderIdentPadding,
}

impl fmt::Display for HeaderIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{}\n{}\n{}\n{}",
            self.magic,
            self.class,
            self.endian,
            self.version,
            self.osabi,
            self.abiversion,
            self.padding,
        )
    }
}

pub fn parse_bytes_header_ident(data: &[u8]) -> Result<HeaderIdent, error::Error> {
    match HeaderIdent::from_bytes((data, 0)) {
        Ok((_rest, val)) => Ok(val),
        Err(deku_err) => {
            let error_affected = String::from("unknown");
            let error_suggestion = String::from("verify the file format");
            let error_type = error::ErrorType::Deku(deku_err);
            let err = error::Error {
                error_type,
                affected: error_affected,
                suggestion: error_suggestion,
            };
            Err(err)
        }
    }
}

impl Default for HeaderIdent {
    fn default() -> Self {
        HeaderIdent {
            magic: header_ident_magic::HeaderIdentMagic::new(),
            class: header_ident_class::HeaderIdentClass::new(),
            endian: header_ident_endian::HeaderIdentEndian::new(),
            version: header_ident_version::HeaderIdentVersion::new(),
            osabi: header_ident_osabi::HeaderIdentOsAbi::new(),
            abiversion: header_ident_abiversion::HeaderIdentAbiVersion::new(),
            padding: header_ident_padding::HeaderIdentPadding::new(),
        }
    }
}

impl HeaderIdent {
    pub fn new() -> Self {
        HeaderIdent {
            magic: header_ident_magic::HeaderIdentMagic::new(),
            class: header_ident_class::HeaderIdentClass::new(),
            endian: header_ident_endian::HeaderIdentEndian::new(),
            version: header_ident_version::HeaderIdentVersion::new(),
            osabi: header_ident_osabi::HeaderIdentOsAbi::new(),
            abiversion: header_ident_abiversion::HeaderIdentAbiVersion::new(),
            padding: header_ident_padding::HeaderIdentPadding::new(),
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

    #[test]
    fn test_from_bytes() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x06, 0x07,
        ];
        let (rest, val) = HeaderIdent::from_bytes((&data, 0)).unwrap();
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
            ei_pad: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
        };
        let expected = HeaderIdent {
            magic: header_ident_magic,
            class: header_ident_class,
            endian: header_ident_endian,
            version: header_ident_version,
            osabi: header_ident_osabi,
            abiversion: header_ident_abiversion,
            padding: header_ident_padding,
        };

        assert_eq!(expected, val);

        let data_out = val.to_bytes().unwrap();
        assert_eq!(data, data_out);

        assert_eq!((vec![].as_ref(), 0 as usize), rest);
    }

    #[test]
    fn test_from_bytes_empty() {
        let data: Vec<u8> = vec![];

        let err = HeaderIdent::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_incomplete() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x06,
        ];

        let err = HeaderIdent::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_invalid() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x02, 0x03, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x06, 0x07,
        ];

        let err = HeaderIdent::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_try_from() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x00, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17,
        ];

        let val = HeaderIdent::try_from(data.as_ref()).unwrap();
        println!("value: {:?}", val);

        let expected = HeaderIdent::new();
        assert_eq!(expected, val);

        let value: Vec<u8> = val.try_into().unwrap();
        assert_eq!(data, value);
    }
}
