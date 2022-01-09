use crate::display;
use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use std::fmt;

// list copied from https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum HeaderIdentOsAbi {
    #[deku(id = "0x00")]
    SystemV,
    #[deku(id = "0x01")]
    HpUx,
    #[deku(id = "0x02")]
    NetBSD,
    #[deku(id = "0x03")]
    Linux,
    #[deku(id = "0x04")]
    GnuHurd,
    #[deku(id = "0x06")]
    Solaris,
    #[deku(id = "0x07")]
    Aix,
    #[deku(id = "0x08")]
    Irix,
    #[deku(id = "0x09")]
    FreeBSD,
    #[deku(id = "0x0A")]
    Tru64,
    #[deku(id = "0x0B")]
    NovelModesto,
    #[deku(id = "0x0C")]
    OpenBSD,
    #[deku(id = "0x0D")]
    OpenVMS,
    #[deku(id = "0x0E")]
    HPNonStop,
    #[deku(id = "0x0F")]
    Aros,
    #[deku(id = "0x10")]
    FenixOS,
    #[deku(id = "0x11")]
    CloudABI,
    #[deku(id = "0x12")]
    StratusVOS,
    #[deku(id = "0xFF")]
    Standalone,
}

impl fmt::Display for HeaderIdentOsAbi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (data_str, byte): (&str, u8) = match *self {
            HeaderIdentOsAbi::SystemV => ("SystemV", 0x00),
            HeaderIdentOsAbi::HpUx => ("HP-UX", 0x01),
            HeaderIdentOsAbi::NetBSD => ("NetBSD", 0x02),
            HeaderIdentOsAbi::Linux => ("Linux", 0x03),
            HeaderIdentOsAbi::GnuHurd => ("GNU Hurd", 0x04),
            HeaderIdentOsAbi::Solaris => ("Sun Solaris", 0x06),
            HeaderIdentOsAbi::Aix => ("IBM AIX", 0x07),
            HeaderIdentOsAbi::Irix => ("IRIX", 0x08),
            HeaderIdentOsAbi::FreeBSD => ("FreeBSD", 0x09),
            HeaderIdentOsAbi::Tru64 => ("Tru64 UNIX", 0x0A),
            HeaderIdentOsAbi::NovelModesto => ("Novel Modesto", 0x0B),
            HeaderIdentOsAbi::OpenBSD => ("OpenBSD", 0x0C),
            HeaderIdentOsAbi::OpenVMS => ("OpenVMS", 0x0D),
            HeaderIdentOsAbi::HPNonStop => ("HP NonStop", 0x0E),
            HeaderIdentOsAbi::Aros => ("AROS Research Operating System", 0x0F),
            HeaderIdentOsAbi::FenixOS => ("Fenix OX", 0x10),
            HeaderIdentOsAbi::CloudABI => ("CloudABI", 0x11),
            HeaderIdentOsAbi::StratusVOS => ("Stratus VOS", 0x12),
            HeaderIdentOsAbi::Standalone => ("Standalone application", 0xFF),
        };

        let data_vec: Vec<u8> = byte.to_ne_bytes().to_vec();
        display::print_field(f, "OS ABI", data_str, &data_vec)
    }
}

impl Default for HeaderIdentOsAbi {
    fn default() -> Self {
        HeaderIdentOsAbi::SystemV
    }
}

impl HeaderIdentOsAbi {
    pub fn new() -> Self {
        HeaderIdentOsAbi::SystemV
    }

    fn reader(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, Vec<u8>), DekuError> {
        debug!("rest: {:?}", rest);
        debug!("len rest: {}", rest.len());

        // slice off length of options
        let index = 4 * 8;

        // Check split_at precondition
        if index > rest.len() {
            return Err(DekuError::Parse(format!(
                "Not enough data to read HeaderIdentOsAbi. Bits expected: {}, Bits given: {}",
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
        let data: Vec<u8> = vec![0x03];

        let (rest, val) = HeaderIdentOsAbi::from_bytes((&data, 0)).unwrap();
        println!("rest: {:?}", rest);
        println!("value: {:?}", val);

        let expected = HeaderIdentOsAbi::Linux;
        assert_eq!(expected, val);

        assert_eq!((vec![].as_ref(), 0 as usize), rest);
    }

    #[test]
    fn test_from_bytes_empty() {
        let data: Vec<u8> = vec![];

        let err = HeaderIdentOsAbi::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_incomplete() {
        let data: Vec<u8> = vec![];

        let err = HeaderIdentOsAbi::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_from_bytes_invalid() {
        let data: Vec<u8> = vec![0xF0];

        let err = HeaderIdentOsAbi::from_bytes((&data, 0));
        assert!(matches!(err, Err(_)));
    }

    #[test]
    fn test_try_from() {
        let data: Vec<u8> = vec![0x00];

        let val = HeaderIdentOsAbi::try_from(data.as_ref()).unwrap();
        println!("value: {:?}", val);
        let expected = HeaderIdentOsAbi::new();
        assert_eq!(expected, val);

        let val: Vec<u8> = val.try_into().unwrap();
        assert_eq!(data, val);
    }
}
