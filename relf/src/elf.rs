use crate::error;
use deku::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
// #[deku(endian = "little")]
#[deku()]
pub struct HeaderMagic {
    #[deku(bytes = "1")]
    ei_mag0: u8,
    #[deku(bytes = "1")]
    ei_mag1: u8,
    #[deku(bytes = "1")]
    ei_mag2: u8,
    #[deku(bytes = "1")]
    ei_mag3: u8,
}

impl fmt::Display for HeaderMagic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:<10}: {:>12} - 0x{:02x}{:02x}{:02x}{:02x}",
            "MAGIC", "ELF", self.ei_mag0, self.ei_mag1, self.ei_mag2, self.ei_mag3
        )
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
        match *self {
            HeaderClass::Bit32 => write!(f, "{:<10}: {:>12} - 0x{:>02}", "CLASS", "32bit", 0x01),
            HeaderClass::Bit64 => write!(f, "{:<10}: {:>12} - 0x{:>02}", "CLASS", "64bit", 0x02),
        }
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku()]
pub struct FileHeader {
    magic: HeaderMagic,
    class: HeaderClass,
}

impl fmt::Display for FileHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "** HEADER\n{}\n{}", self.magic, self.class)
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
    fn test_parse_file_header() {
        let data: Vec<u8> = vec![0x7F, 0x45, 0x4C, 0x46, 0x01];
        let val = parse_bytes(&data).unwrap();

        let header_magic = HeaderMagic {
            ei_mag0: 0x7F,
            ei_mag1: 0x45,
            ei_mag2: 0x4C,
            ei_mag3: 0x46,
        };
        let headerclass = HeaderClass::Bit32;
        let header = FileHeader {
            magic: header_magic,
            class: headerclass,
        };
        let expected = ElfFile {
            file_header: header,
        };
        assert_eq!(expected, val);

        let data_out = val.to_bytes().unwrap();
        assert_eq!(data, data_out);
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
