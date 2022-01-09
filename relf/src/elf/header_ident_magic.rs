use crate::display;
use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
// #[deku(endian = "little")]
// #[deku(magic = b"\x7FELF")]
// #[deku(magic = b"\x7F")]
#[deku()]
pub struct HeaderIdentMagic {
    /*
      #[deku(bytes = "1", assert_eq = "0x45")]
      pub ei_mag1: u8,
      #[deku(bytes = "1", assert_eq = "0x4C")]
      pub ei_mag2: u8,
      #[deku(bytes = "1", assert_eq = "0x46")]
      pub ei_mag3: u8,
    */
    // assert_eq = "0x7F454C46",
    #[deku(
        bytes = "1",
        count = "4",
        endian = "little",
        reader = "HeaderIdentMagic::reader(deku::rest)"
    )]
    pub ei_mag: Vec<u8>,
    /*
      #[deku(
          count = "4",
          endian = "little",
      )]
      pub ei_mag: u32
    */
}

impl fmt::Display for HeaderIdentMagic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /*
          let data_vec: Vec<u8> = vec![self.ei_mag0, self.ei_mag1, self.ei_mag2, self.ei_mag3];
          let data_str: &str = "ELF";
          display::print_field(f, "MAGIC", data_str, &data_vec)
        */
        display::print_field(f, "VERSION", "ELF", &self.ei_mag)
    }
}

impl Default for HeaderIdentMagic {
    fn default() -> Self {
        HeaderIdentMagic {
            ei_mag: vec![0x7F, 0x45, 0x4C, 0x46],
        }
    }
}

impl HeaderIdentMagic {
    pub fn new() -> Self {
        HeaderIdentMagic {
            ei_mag: vec![0x7F, 0x45, 0x4C, 0x46],
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
                "Not enough data to read HeaderIdentMagic. Bits expected: {}, Bits given: {}",
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

        // verifying return
        if ret != vec![0x7F, 0x45, 0x4C, 0x46] {
            return Err(DekuError::Parse("HeaderIdentMagic != 0x7FELF".to_string()));
        }
        Ok((rest, ret))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bytes() {
        let data: Vec<u8> = vec![0x7F, 0x45, 0x4C, 0x46];

        let (rest, val) = HeaderIdentMagic::from_bytes((&data, 0)).unwrap();
        println!("rest: {:?}", rest);
        println!("value: {:?}", val);

        let expected = HeaderIdentMagic {
            ei_mag: vec![0x7F, 0x45, 0x4C, 0x46],
        };
        assert_eq!(expected, val);

        assert_eq!((vec![].as_ref(), 0 as usize), rest);
    }

    #[test]
    fn test_from_bytes_empty() {
        let data: Vec<u8> = vec![];

        let err = HeaderIdentMagic::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_incomplete() {
        let data: Vec<u8> = vec![0x7F, 0x45, 0x4C];

        let err = HeaderIdentMagic::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_invalid() {
        let data: Vec<u8> = vec![0x7F, 0x45, 0x4C, 0x47];

        let err = HeaderIdentMagic::from_bytes((&data, 0));
        println!("{:?}", err);
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_try_from() {
        let data: Vec<u8> = vec![0x7F, 0x45, 0x4C, 0x46];

        let val = HeaderIdentMagic::try_from(data.as_ref()).unwrap();
        println!("value: {:?}", val);

        let expected = HeaderIdentMagic::new();
        assert_eq!(expected, val);

        let value: Vec<u8> = val.try_into().unwrap();
        assert_eq!(data, value);
    }
}
