use crate::display;
use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u16", bytes = "2", endian = "little")]
//        reader = "HeaderType::reader(deku::rest)"
pub enum HeaderType {
    #[deku(id = "0x0000")]
    None,
    #[deku(id = "0x0001")]
    Rel,
    #[deku(id = "0x0002")]
    Exec,
    #[deku(id = "0x0003")]
    Dyn,
    #[deku(id = "0x0004")]
    Core,
}

impl fmt::Display for HeaderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (data_str, byte): (&str, u16) = match *self {
            HeaderType::None => ("No file type", 0x0000),
            HeaderType::Rel => ("Relocatable file", 0x0100),
            HeaderType::Exec => ("Executable file", 0x0200),
            HeaderType::Dyn => ("Shared object file", 0x0300),
            HeaderType::Core => ("Core file", 0x0400),
        };
        let data_vec: Vec<u8> = byte.to_ne_bytes().to_vec();
        display::print_field(f, "TYPE", data_str, &data_vec)
    }
}

impl Default for HeaderType {
    fn default() -> Self {
        HeaderType::None
    }
}

impl HeaderType {
    pub fn new() -> Self {
        HeaderType::None
    }

    fn reader(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, Vec<u8>), DekuError> {
        debug!("rest: {:?}", rest);
        debug!("len rest: {}", rest.len());

        // slice off length of options
        let index = 4 * 8;

        // Check split_at precondition
        if index > rest.len() {
            return Err(DekuError::Parse(format!(
                "Not enough data to read HeaderType. Bits expected: {}, Bits given: {}",
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
        let data: Vec<u8> = vec![0x03, 0x00];

        let (rest, val) = HeaderType::from_bytes((&data, 0)).unwrap();
        println!("rest: {:?}", rest);
        println!("value: {:?}", val);

        let expected = HeaderType::Dyn;
        assert_eq!(expected, val);

        assert_eq!((vec![].as_ref(), 0 as usize), rest);
    }

    #[test]
    fn test_from_bytes_empty() {
        let data: Vec<u8> = vec![];

        let err = HeaderType::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_incomplete() {
        let data: Vec<u8> = vec![0x03];

        let err = HeaderType::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_invalid() {
        let data: Vec<u8> = vec![0xFF, 0xFF];

        let err = HeaderType::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_try_from() {
        let data: Vec<u8> = vec![0x00, 0x00];

        let val = HeaderType::try_from(data.as_ref()).unwrap();
        println!("value: {:?}", val);
        let expected = HeaderType::new();
        assert_eq!(expected, val);

        let val: Vec<u8> = val.try_into().unwrap();
        assert_eq!(data, val);
    }
}
