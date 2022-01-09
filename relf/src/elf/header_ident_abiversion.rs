use crate::display;
use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku()]
pub struct HeaderIdentAbiVersion {
    #[deku(bytes = "1")]
    pub ei_abiversion: u8,
}

impl fmt::Display for HeaderIdentAbiVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data_vec: Vec<u8> = self.ei_abiversion.to_ne_bytes().to_vec();
        let data_str: &str = "unspecified";
        display::print_field(f, "ABI VERSION", data_str, &data_vec)
    }
}

impl Default for HeaderIdentAbiVersion {
    fn default() -> Self {
        HeaderIdentAbiVersion {
            ei_abiversion: 0x01,
        }
    }
}

impl HeaderIdentAbiVersion {
    pub fn new() -> Self {
        HeaderIdentAbiVersion {
            ei_abiversion: 0x01,
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
                "Not enough data to read HeaderIdentAbiVersion. Bits expected: {}, Bits given: {}",
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
        let data: Vec<u8> = vec![0x02];

        let (rest, val) = HeaderIdentAbiVersion::from_bytes((&data, 0)).unwrap();
        println!("rest: {:?}", rest);
        println!("value: {:?}", val);

        let expected = HeaderIdentAbiVersion {
            ei_abiversion: 0x02,
        };
        assert_eq!(expected, val);

        assert_eq!((vec![].as_ref(), 0 as usize), rest);
    }

    #[test]
    fn test_from_bytes_empty() {
        let data: Vec<u8> = vec![];

        let err = HeaderIdentAbiVersion::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_incomplete() {
        let data: Vec<u8> = vec![];

        let err = HeaderIdentAbiVersion::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_try_from() {
        let data: Vec<u8> = vec![0x01];

        let val = HeaderIdentAbiVersion::try_from(data.as_ref()).unwrap();
        println!("value: {:?}", val);

        let expected = HeaderIdentAbiVersion::new();
        assert_eq!(expected, val);

        let value: Vec<u8> = val.try_into().unwrap();
        assert_eq!(data, value);
    }
}
