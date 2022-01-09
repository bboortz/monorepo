use crate::display;
use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku()]
pub struct HeaderIdentPadding {
    #[deku(
        bytes = "1",
        count = "7",
        reader = "HeaderIdentPadding::reader(deku::rest)"
    )]
    pub ei_pad: Vec<u8>,
}

impl fmt::Display for HeaderIdentPadding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display::print_field(f, "PADDING", "7 Bytes Padding", &self.ei_pad)
    }
}

impl Default for HeaderIdentPadding {
    fn default() -> Self {
        HeaderIdentPadding {
            ei_pad: vec![0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17],
        }
    }
}

impl HeaderIdentPadding {
    pub fn new() -> Self {
        HeaderIdentPadding {
            ei_pad: vec![0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17],
        }
    }

    fn reader(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, Vec<u8>), DekuError> {
        debug!("rest: {:?}", rest);
        debug!("len rest: {}", rest.len());

        // slice off length of options
        let index = 7 * 8;

        // Check split_at precondition
        if index > rest.len() {
            return Err(DekuError::Parse(format!(
                "Not enough data to read HeaderIdentPadding. Bits expected: {}, Bits given: {}",
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
        let data: Vec<u8> = vec![0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17];

        let (rest, val) = HeaderIdentPadding::from_bytes((&data, 0)).unwrap();
        println!("rest: {:?}", rest);
        println!("value: {:?}", val);

        let expected = HeaderIdentPadding {
            ei_pad: vec![0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17],
        };
        assert_eq!(expected, val);

        assert_eq!((vec![].as_ref(), 0 as usize), rest);
    }

    #[test]
    fn test_from_bytes_empty() {
        let data: Vec<u8> = vec![];

        let err = HeaderIdentPadding::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_incomplete() {
        let data: Vec<u8> = vec![0x11, 0x12, 0x13, 0x14, 0x15, 0x16];

        let err = HeaderIdentPadding::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_try_from() {
        let data: Vec<u8> = vec![0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17];

        let val = HeaderIdentPadding::try_from(data.as_ref()).unwrap();
        println!("value: {:?}", val);

        let expected = HeaderIdentPadding::new();
        assert_eq!(expected, val);

        let value: Vec<u8> = val.try_into().unwrap();
        assert_eq!(data, value);
    }
}
