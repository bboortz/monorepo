use crate::display;
use deku::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum HeaderIdentClass {
    #[deku(id = "0x01")]
    Bit32,
    #[deku(id = "0x02")]
    Bit64,
}

impl fmt::Display for HeaderIdentClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (data_str, byte): (&str, u8) = match *self {
            HeaderIdentClass::Bit32 => ("32bit", 0x01),
            HeaderIdentClass::Bit64 => ("64bit", 0x02),
        };
        let data_vec: Vec<u8> = byte.to_ne_bytes().to_vec();
        display::print_field(f, "CLASS", data_str, &data_vec)
    }
}

impl Default for HeaderIdentClass {
    fn default() -> Self {
        HeaderIdentClass::Bit32
    }
}

impl HeaderIdentClass {
    pub fn new() -> Self {
        HeaderIdentClass::Bit32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bytes() {
        let data: Vec<u8> = vec![0x02];

        let (rest, val) = HeaderIdentClass::from_bytes((&data, 0)).unwrap();
        println!("rest: {:?}", rest);
        println!("value: {:?}", val);

        let expected = HeaderIdentClass::Bit64;
        assert_eq!(expected, val);

        assert_eq!((vec![].as_ref(), 0 as usize), rest);
    }

    #[test]
    fn test_from_bytes_empty() {
        let data: Vec<u8> = vec![];

        let err = HeaderIdentClass::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_invalid() {
        let data: Vec<u8> = vec![0xFF];

        let err = HeaderIdentClass::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_try_from() {
        let data: Vec<u8> = vec![0x01];

        let val = HeaderIdentClass::try_from(data.as_ref()).unwrap();
        println!("value: {:?}", val);
        let expected = HeaderIdentClass::new();
        assert_eq!(expected, val);

        let val: Vec<u8> = val.try_into().unwrap();
        assert_eq!(data, val);
    }
}
