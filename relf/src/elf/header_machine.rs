use crate::display;
use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use std::fmt;

// TODO: extend num list of all possible machines
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u16", bytes = "2", endian = "little")]
pub enum HeaderMachine {
    #[deku(id = "0x0000")]
    None,
    #[deku(id = "0x0001")]
    WE32100,
    #[deku(id = "0x0002")]
    Sparc,
    #[deku(id = "0x0003")]
    X86,
    #[deku(id = "0x0004")]
    Motorola68000,
    #[deku(id = "0x0005")]
    Motorola88000,
    #[deku(id = "0x0006")]
    IntelMCU,
    #[deku(id = "0x0007")]
    Intel80860,
    #[deku(id = "0x003E")]
    X86_64,
}

impl fmt::Display for HeaderMachine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (data_str, byte): (&str, u16) = match *self {
            HeaderMachine::None => ("unspecified instruction set", 0x0000),
            HeaderMachine::WE32100 => ("AT&T WE 32100", 0x0100),
            HeaderMachine::Sparc => ("SUN Sparc", 0x0200),
            HeaderMachine::X86 => ("x86", 0x0300),
            HeaderMachine::Motorola68000 => ("Motorola 68000 (m68k)", 0x0400),
            HeaderMachine::Motorola88000 => ("Motorola 88000 (m88k)", 0x0500),
            HeaderMachine::IntelMCU => ("Intel MCU", 0x0600),
            HeaderMachine::Intel80860 => ("Intel 80860", 0x0700),
            HeaderMachine::X86_64 => ("AMD x86-64", 0x003E),
        };
        let data_vec: Vec<u8> = byte.to_ne_bytes().to_vec();
        display::print_field(f, "MACHINE", data_str, &data_vec)
    }
}

impl Default for HeaderMachine {
    fn default() -> Self {
        HeaderMachine::None
    }
}

impl HeaderMachine {
    pub fn new() -> Self {
        HeaderMachine::None
    }

    fn reader(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, Vec<u8>), DekuError> {
        debug!("rest: {:?}", rest);
        debug!("len rest: {}", rest.len());

        // slice off length of options
        let index = 4 * 8;

        // Check split_at precondition
        if index > rest.len() {
            return Err(DekuError::Parse(format!(
                "Not enough data to read HeaderMachine. Bits expected: {}, Bits given: {}",
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
        let data: Vec<u8> = vec![0x3E, 0x00];

        let (rest, val) = HeaderMachine::from_bytes((&data, 0)).unwrap();
        println!("rest: {:?}", rest);
        println!("value: {:?}", val);

        let expected = HeaderMachine::X86_64;
        assert_eq!(expected, val);

        assert_eq!((vec![].as_ref(), 0 as usize), rest);
    }

    #[test]
    fn test_from_bytes_empty() {
        let data: Vec<u8> = vec![];

        let err = HeaderMachine::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_incomplete() {
        let data: Vec<u8> = vec![0x03];

        let err = HeaderMachine::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_invalid() {
        let data: Vec<u8> = vec![0xFF, 0xFF];

        let err = HeaderMachine::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_try_from() {
        let data: Vec<u8> = vec![0x00, 0x00];

        let val = HeaderMachine::try_from(data.as_ref()).unwrap();
        println!("value: {:?}", val);
        let expected = HeaderMachine::new();
        assert_eq!(expected, val);

        let val: Vec<u8> = val.try_into().unwrap();
        assert_eq!(data, val);
    }
}
