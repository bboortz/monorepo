use crate::display;
use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku()]
pub struct HeaderVersion {
    #[deku(
        bytes = "1",
        count = "4",
        endian = "little",
        reader = "HeaderVersion::reader(deku::rest)"
    )]
    pub ei_version: Vec<u8>,
}

impl fmt::Display for HeaderVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display::print_field(f, "VERSION", "version", &self.ei_version)
    }
}

impl Default for HeaderVersion {
    fn default() -> Self {
        HeaderVersion {
            ei_version: vec![0x01, 0x02, 0x03, 0x04],
        }
    }
}

impl HeaderVersion {
    pub fn new() -> Self {
        HeaderVersion {
            ei_version: vec![0x01, 0x02, 0x03, 0x04],
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
                "Not enough data to read HeaderVersion. Bits expected: {}, Bits given: {}",
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
        let data: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];

        let (rest, val) = HeaderVersion::from_bytes((&data, 0)).unwrap();
        println!("rest: {:?}", rest);
        println!("value: {:?}", val);

        let expected = HeaderVersion {
            ei_version: vec![0x01, 0x02, 0x03, 0x04],
        };
        assert_eq!(expected, val);

        assert_eq!((vec![].as_ref(), 0 as usize), rest);
    }

    #[test]
    fn test_from_bytes_empty() {
        let data: Vec<u8> = vec![];

        let err = HeaderVersion::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_incomplete() {
        let data: Vec<u8> = vec![0x01, 0x02, 0x03];

        let err = HeaderVersion::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_try_from() {
        let data: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];

        let val = HeaderVersion::try_from(data.as_ref()).unwrap();
        println!("value: {:?}", val);

        let expected = HeaderVersion::new();
        assert_eq!(expected, val);

        let value: Vec<u8> = val.try_into().unwrap();
        assert_eq!(data, value);
    }
}
