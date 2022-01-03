use crate::display;
use crate::error;
use deku::prelude::*;
use std::fmt;
extern crate hexplay;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
// #[deku(endian = "little")]
// #[deku(magic = b"\x7fELF")]
#[deku()]
pub struct HeaderMagic {
    #[deku(bytes = "1", assert_eq = "0x7F")]
    ei_mag0: u8,
    #[deku(bytes = "1", assert_eq = "0x45")]
    ei_mag1: u8,
    #[deku(bytes = "1", assert_eq = "0x4C")]
    ei_mag2: u8,
    #[deku(bytes = "1", assert_eq = "0x46")]
    ei_mag3: u8,
}

impl fmt::Display for HeaderMagic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // let data_vec: Vec<u8> = self.ei_abiversion.to_ne_bytes().to_vec();
        let mut data_vec: Vec<u8> = vec![];
        data_vec.push(self.ei_mag0);
        data_vec.push(self.ei_mag1);
        data_vec.push(self.ei_mag2);
        data_vec.push(self.ei_mag3);
        let data_str: &str = "ELF";
        display::print_field(f, "MAGIC", data_str, &data_vec)
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum HeaderClass {
    #[deku(id = "0x01")]
    Bit32,
    #[deku(id = "0x02")]
    Bit64,
}

impl fmt::Display for HeaderClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (data_str, byte): (&str, u8) = match *self {
            // let StringByteTransfer{ string: data_str, byte: byte } = match *self {
            HeaderClass::Bit32 => ("32bit", 0x01),
            HeaderClass::Bit64 => ("64bit", 0x02),
        };
        let data_vec: Vec<u8> = byte.to_ne_bytes().to_vec();
        display::print_field(f, "CLASS", data_str, &data_vec)
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum HeaderEndian {
    #[deku(id = "0x01")]
    Little,
    #[deku(id = "0x02")]
    Big,
}

impl fmt::Display for HeaderEndian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (data_str, byte): (&str, u8) = match *self {
            HeaderEndian::Little => ("Little", 0x01),
            HeaderEndian::Big => ("Big", 0x02),
        };
        let data_vec: Vec<u8> = byte.to_ne_bytes().to_vec();
        display::print_field(f, "ENDIAN", data_str, &data_vec)

        /*
        match *self {
            HeaderEndian::Little => {
                write!(f, "{:<14}: {:>14} - 0x{:>02}", "ENDIAN", "Little", 0x01)
            }
            HeaderEndian::Big => write!(f, "{:<14}: {:>14} - 0x{:>02}", "ENDIAN", "Big", 0x02),
        }
                */
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku()]
pub struct HeaderVersion {
    #[deku(bytes = "1", assert_eq = "0x01")]
    ei_version: u8,
}

impl fmt::Display for HeaderVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /*
        let (data_str, byte): (&str, u8) = match *self {
            HeaderEndian::Little => ("Little", 0x01),
            HeaderEndian::Big => ("Big", 0x02),
        };
        let data_vec: Vec<u8> = byte.to_ne_bytes().to_vec();
                */

        let data_vec: Vec<u8> = self.ei_version.to_ne_bytes().to_vec();
        let data_str: &str = "1";
        display::print_field(f, "VERSION", data_str, &data_vec)
        /*
        write!(
            f,
            "{:<14}: {:>14} - 0x{:02x}",
            "Version", self.ei_version, self.ei_version
        )
                */
    }
}

// list copied from https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum HeaderOsAbi {
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
}

impl fmt::Display for HeaderOsAbi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (data_str, byte): (&str, u8) = match *self {
            HeaderOsAbi::SystemV => ("SystemV", 0x00),
            HeaderOsAbi::HpUx => ("HP-UX", 0x01),
            HeaderOsAbi::NetBSD => ("NetBSD", 0x02),
            HeaderOsAbi::Linux => ("Linux", 0x03),
            HeaderOsAbi::GnuHurd => ("GNU Hurd", 0x04),
            HeaderOsAbi::Solaris => ("Sun Solaris", 0x06),
            HeaderOsAbi::Aix => ("IBM AIX", 0x07),
            HeaderOsAbi::Irix => ("IRIX", 0x08),
            HeaderOsAbi::FreeBSD => ("FreeBSD", 0x09),
            HeaderOsAbi::Tru64 => ("Tru64 UNIX", 0x0A),
            HeaderOsAbi::NovelModesto => ("Novel Modesto", 0x0B),
            HeaderOsAbi::OpenBSD => ("OpenBSD", 0x0C),
            HeaderOsAbi::OpenVMS => ("OpenVMS", 0x0D),
            HeaderOsAbi::HPNonStop => ("HP NonStop", 0x0E),
            HeaderOsAbi::Aros => ("AROS Research Operating System", 0x0F),
            HeaderOsAbi::FenixOS => ("Fenix OX", 0x10),
            HeaderOsAbi::CloudABI => ("CloudABI", 0x11),
            HeaderOsAbi::StratusVOS => ("Stratus VOS", 0x12),
        };

        let data_vec: Vec<u8> = byte.to_ne_bytes().to_vec();
        display::print_field(f, "ENDIAN", data_str, &data_vec)
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku()]
pub struct HeaderAbiVersion {
    #[deku(bytes = "1")]
    ei_abiversion: u8,
}

impl fmt::Display for HeaderAbiVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /*
        let src2: Vec<u8> = br#"e{"ddie"}"#.to_vec();
        // to String
        // from_utf8 consume the vector of bytes
        let string2: String = String::from_utf8(src2.clone()).unwrap();
        // to str
        let str2: &str = std::str::from_utf8(&src2).unwrap();
        // to vec of chars
        let char2: Vec<char> = src2.iter().map(|b| *b as char).collect::<Vec<_>>();
        println!(
            "Vec<u8>:{:?} | String:{:?}, str:{:?}, Vec<char>:{:?}",
            src2, string2, str2, char2
        );
                */

        /*
        let data_vec: Vec<u8> = vec![];
        data_vec.push(self.ei_abiversion);
                */
        let data_vec: Vec<u8> = self.ei_abiversion.to_ne_bytes().to_vec();
        let data_str: &str = std::str::from_utf8(&data_vec).unwrap();
        let data_str: &str = "unspecified";
        //        let sparkle_heart = str::from_utf8(&data2).unwrap();

        display::print_field(f, "ABI VERSION", data_str, &data_vec)
        /*
        write!(
            f,
            "{:<14}: {:>14} - 0x{:02x}",
            "ABI Version", self.ei_abiversion, self.ei_abiversion
        )
                */
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku()]
pub struct HeaderPadding {
    #[deku(bytes = "1", count = "7")]
    ei_pad: Vec<u8>,
}

impl fmt::Display for HeaderPadding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display::print_field(f, "PADDING", "7 Bytes Padding", &self.ei_pad)
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku()]
pub struct FileHeader {
    magic: HeaderMagic,
    class: HeaderClass,
    endian: HeaderEndian,
    version: HeaderVersion,
    osabi: HeaderOsAbi,
    abiversion: HeaderAbiVersion,
    padding: HeaderPadding,
}

impl fmt::Display for FileHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "** HEADER\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
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

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku()]
pub struct ElfFile {
    file_header: FileHeader,
}

impl fmt::Display for ElfFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.file_header)
    }
}

/*
pub fn parse_header_magic(data: &[u8]) -> Result<HeaderMagic, error::Error> {
    let (_rest, mut val) = HeaderMagic::from_bytes((data.as_ref(), 0)).unwrap();
    Ok(val)
}

pub fn parse_header_class(data: &[u8]) -> Result<HeaderClass, error::Error> {
    match HeaderClass::from_bytes((data.as_ref(), 0)) {
        Ok((_rest, val)) => Ok(val),
        Err(deku_err) => {
            let error_affected = String::from("unknown");
            let error_suggestion = String::from("testcase");
            let error_type = error::ErrorType::Deku(deku_err);
            let err = error::Error {
                error_type: error_type,
                affected: error_affected,
                suggestion: error_suggestion,
            };
            Err(err)
        }
    }
}

pub fn parse_file_header(data: &[u8]) -> Result<FileHeader, error::Error> {
    let (_rest, mut val) = FileHeader::from_bytes((data.as_ref(), 0)).unwrap();
    Ok(val)
}
*/

pub fn parse_bytes(data: &[u8]) -> Result<ElfFile, error::Error> {
    match ElfFile::from_bytes((data, 0)) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bytes() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x06, 0x07,
        ];
        let val = parse_bytes(&data).unwrap();

        let header_magic = HeaderMagic {
            ei_mag0: 0x7F,
            ei_mag1: 0x45,
            ei_mag2: 0x4C,
            ei_mag3: 0x46,
        };
        let header_class = HeaderClass::Bit32;
        let header_endian = HeaderEndian::Little;
        let header_version = HeaderVersion { ei_version: 0x01 };
        let header_osabi = HeaderOsAbi::Linux;
        let header_abiversion = HeaderAbiVersion {
            ei_abiversion: 0x01,
        };
        let header_padding = HeaderPadding {
            ei_pad: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
        };
        let header = FileHeader {
            magic: header_magic,
            class: header_class,
            endian: header_endian,
            version: header_version,
            osabi: header_osabi,
            abiversion: header_abiversion,
            padding: header_padding,
        };
        let expected = ElfFile {
            file_header: header,
        };
        assert_eq!(expected, val);

        let data_out = val.to_bytes().unwrap();
        assert_eq!(data, data_out);
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

    /*
    #[test]
    fn test_parse_header_magic() {
        let data: Vec<u8> = vec![0x7F, 0x45, 0x4C, 0x46];
        let val = parse_header_magic(&data).unwrap();

        assert_eq!(
            HeaderMagic {
                ei_mag0: 0x7F,
                ei_mag1: 0x45,
                ei_mag2: 0x4C,
                ei_mag3: 0x46,
            },
            val
        );
        // assert_eq!(0, rest);

        let data_out = val.to_bytes().unwrap();
        assert_eq!(data, data_out);

        /*
        assert_eq!(output, Some((expected.as_slice(), remainder.as_slice())));
        match parse_header_magic(data) {
          Ok(val) => {},
          Err(e) => {}
        }
        parse_bytes(data);
        */
    }

    #[test]
    fn test_parse_header_class() {
        let data: Vec<u8> = vec![0x01];
        let val = parse_header_class(&data).unwrap();

        let headerclass = HeaderClass::Bit32;
        assert_eq!(headerclass, val);

        let data_out = val.to_bytes().unwrap();
        assert_eq!(data, data_out);
    }

    #[test]
    fn test_parse_header_class_err() {
        let data: Vec<u8> = vec![0x00];
        let val = parse_header_class(&data);

        match val {
            Ok(_) => assert!(false, "Need to return an Error!"),
            Err(e) => {
                println!("{}", e);
                println!("{:?}", e);
                println!("{}", e.error_type);
                println!("{:?}", e.error_type);
                assert!(true);
            }
        }
    }

    #[test]
    fn test_parse_file_header() {
        let data: Vec<u8> = vec![0x7F, 0x45, 0x4C, 0x46, 0x01];
        let val = parse_file_header(&data).unwrap();

        let header_magic = HeaderMagic {
            ei_mag0: 0x7F,
            ei_mag1: 0x45,
            ei_mag2: 0x4C,
            ei_mag3: 0x46,
        };
        let headerclass = HeaderClass::Bit32;
        let expected = FileHeader {
            magic: header_magic,
            class: headerclass,
        };
        assert_eq!(expected, val);

        let data_out = val.to_bytes().unwrap();
        assert_eq!(data, data_out);
    }
    */

    /*
    #[test]
    fn test_parse_bytes_2() {
        let data: Vec<u8> = vec![0x7F, 0x45, 0x4C, 0x46, 0x02];
        let data: Vec<u8> = vec![0x7F, 0x45, 0x4C, 0x46, 0x02, 0x01, 0x01, 0xFF, 0x00];
        let expected: Vec<u8> = vec![0x02, 0x01, 0x01];
        let remainder: Vec<u8> = vec![0xff, 0x00];
        let output: Option<_> = extract_from_data(&data);
        assert_eq!(output, Some((expected.as_slice(), remainder.as_slice())));
    }

    #[test]
    fn successfully_parses_subsequent_blocks() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x02, 0x01, 0x01, 0xFF, 0x00, 0x7F, 0x45, 0x4C, 0x46, 0x02,
            0x01, 0x01, 0xFF, 0x00,
        ];
        let mut blocks: Vec<&[u8]> = vec![];
        let mut current_state = data.as_slice();
        while let Some((block, remainder)) = extract_from_data(current_state) {
            blocks.push(block);
            current_state = remainder;
        }
        let catch: Vec<u8> = vec![0x02, 0x01, 0x01];
        assert_eq!(blocks.len(), 2);
        for i in blocks {
            assert_eq!(i, catch.as_slice());
        }
        //assert_eq!(output, Some(expected.as_slice()));
    }
    */
}
