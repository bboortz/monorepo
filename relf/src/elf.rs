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
        let data_vec: Vec<u8> = vec![self.ei_mag0, self.ei_mag1, self.ei_mag2, self.ei_mag3];
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
        let data_vec: Vec<u8> = self.ei_version.to_ne_bytes().to_vec();
        let data_str: &str = "1";
        display::print_field(f, "VERSION", data_str, &data_vec)
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
    #[deku(id = "0xFF")]
    Standalone,
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
            HeaderOsAbi::Standalone => ("Standalone application", 0x12),
        };

        let data_vec: Vec<u8> = byte.to_ne_bytes().to_vec();
        display::print_field(f, "OS ABI", data_str, &data_vec)
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
        let data_vec: Vec<u8> = self.ei_abiversion.to_ne_bytes().to_vec();
        let data_str: &str = "unspecified";
        display::print_field(f, "ABI VERSION", data_str, &data_vec)
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
#[deku(type = "u8")]
pub enum HeaderType {
    #[deku(id = "0x00")]
    None,
    #[deku(id = "0x01")]
    Rel,
    #[deku(id = "0x02")]
    Exec,
    #[deku(id = "0x03")]
    Dyn,
    #[deku(id = "0x04")]
    Core,
}

impl fmt::Display for HeaderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (data_str, byte): (&str, u8) = match *self {
            HeaderType::None => ("No file type", 0x00),
            HeaderType::Rel => ("Relocatable file", 0x01),
            HeaderType::Exec => ("Executable file", 0x02),
            HeaderType::Dyn => ("Shared object file", 0x03),
            HeaderType::Core => ("Core file", 0x04),
        };
        let data_vec: Vec<u8> = byte.to_ne_bytes().to_vec();
        display::print_field(f, "TYPE", data_str, &data_vec)
    }
}

// TODO: extend num list of all possible machines
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum HeaderMachine {
    #[deku(id = "0x00")]
    None,
    #[deku(id = "0x01")]
    WE32100,
    #[deku(id = "0x02")]
    Sparc,
    #[deku(id = "0x03")]
    X86,
    #[deku(id = "0x04")]
    Motorola68000,
    #[deku(id = "0x05")]
    Motorola88000,
    #[deku(id = "0x06")]
    IntelMCU,
    #[deku(id = "0x07")]
    Intel80860,
    #[deku(id = "0x3E")]
    X86_64,
}

impl fmt::Display for HeaderMachine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (data_str, byte): (&str, u8) = match *self {
            HeaderMachine::None => ("No file type", 0x00),
            HeaderMachine::WE32100 => ("AT&T WE 32100", 0x01),
            HeaderMachine::Sparc => ("SUN Sparc", 0x02),
            HeaderMachine::X86 => ("x86", 0x03),
            HeaderMachine::Motorola68000 => ("Motorola 68000 (m68k)", 0x04),
            HeaderMachine::Motorola88000 => ("Motorola 88000 (m88k)", 0x05),
            HeaderMachine::IntelMCU => ("Intel MCU", 0x06),
            HeaderMachine::Intel80860 => ("Intel 80860", 0x07),
            HeaderMachine::X86_64 => ("AMD x86-64", 0x3E),
        };
        let data_vec: Vec<u8> = byte.to_ne_bytes().to_vec();
        display::print_field(f, "MACHINE", data_str, &data_vec)
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
    r#type: HeaderType,
    machine: HeaderMachine,
}

impl fmt::Display for FileHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "** HEADER\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            self.magic,
            self.class,
            self.endian,
            self.version,
            self.osabi,
            self.abiversion,
            self.padding,
            self.r#type,
            self.machine,
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
            0x06, 0x07, 0x03, 0x3E,
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
        let header_type = HeaderType::Dyn;
        let header_machine = HeaderMachine::X86_64;

        let header = FileHeader {
            magic: header_magic,
            class: header_class,
            endian: header_endian,
            version: header_version,
            osabi: header_osabi,
            abiversion: header_abiversion,
            padding: header_padding,
            r#type: header_type,
            machine: header_machine,
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

    #[test]
    fn test_parse_bytes_incomplete_type() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x06, 0x07,
        ];
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
    fn test_parse_bytes_incomplete_machine() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x06, 0x07, 0x03,
        ];
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
}
