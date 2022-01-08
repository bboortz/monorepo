use crate::display;
use crate::error;
use deku::prelude::*;
use std::fmt;
extern crate hexplay;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
// #[deku(endian = "little")]
// #[deku(magic = b"\x7fELF")]
#[deku()]
pub struct HeaderIdentMagic {
    #[deku(bytes = "1", assert_eq = "0x7F")]
    ei_mag0: u8,
    #[deku(bytes = "1", assert_eq = "0x45")]
    ei_mag1: u8,
    #[deku(bytes = "1", assert_eq = "0x4C")]
    ei_mag2: u8,
    #[deku(bytes = "1", assert_eq = "0x46")]
    ei_mag3: u8,
}

impl fmt::Display for HeaderIdentMagic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data_vec: Vec<u8> = vec![self.ei_mag0, self.ei_mag1, self.ei_mag2, self.ei_mag3];
        let data_str: &str = "ELF";
        display::print_field(f, "MAGIC", data_str, &data_vec)
    }
}

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

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum HeaderIdentEndian {
    #[deku(id = "0x01")]
    Little,
    #[deku(id = "0x02")]
    Big,
}

impl fmt::Display for HeaderIdentEndian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (data_str, byte): (&str, u8) = match *self {
            HeaderIdentEndian::Little => ("Little", 0x01),
            HeaderIdentEndian::Big => ("Big", 0x02),
        };
        let data_vec: Vec<u8> = byte.to_ne_bytes().to_vec();
        display::print_field(f, "ENDIAN", data_str, &data_vec)
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku()]
pub struct HeaderIdentVersion {
    #[deku(bytes = "1", assert_eq = "0x01")]
    ei_version: u8,
}

impl fmt::Display for HeaderIdentVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data_vec: Vec<u8> = self.ei_version.to_ne_bytes().to_vec();
        let data_str: &str = "1";
        display::print_field(f, "VERSION", data_str, &data_vec)
    }
}

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
            HeaderIdentOsAbi::Standalone => ("Standalone application", 0x12),
        };

        let data_vec: Vec<u8> = byte.to_ne_bytes().to_vec();
        display::print_field(f, "OS ABI", data_str, &data_vec)
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku()]
pub struct HeaderIdentAbiVersion {
    #[deku(bytes = "1")]
    ei_abiversion: u8,
}

impl fmt::Display for HeaderIdentAbiVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data_vec: Vec<u8> = self.ei_abiversion.to_ne_bytes().to_vec();
        let data_str: &str = "unspecified";
        display::print_field(f, "ABI VERSION", data_str, &data_vec)
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku()]
pub struct HeaderIdentPadding {
    #[deku(bytes = "1", count = "7")]
    ei_pad: Vec<u8>,
}

impl fmt::Display for HeaderIdentPadding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display::print_field(f, "PADDING", "7 Bytes Padding", &self.ei_pad)
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku()]
pub struct FileHeaderIdent {
    magic: HeaderIdentMagic,
    class: HeaderIdentClass,
    endian: HeaderIdentEndian,
    version: HeaderIdentVersion,
    osabi: HeaderIdentOsAbi,
    abiversion: HeaderIdentAbiVersion,
    padding: HeaderIdentPadding,
}

impl fmt::Display for FileHeaderIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{}\n{}\n{}\n{}",
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
#[deku(type = "u16", bytes = "2", endian = "little")]
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

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku()]
pub struct HeaderVersion {
    #[deku(bytes = "1", count = "4")]
    ei_version: Vec<u8>,
}

impl fmt::Display for HeaderVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display::print_field(f, "VERSION", "version", &self.ei_version)
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku()]
pub struct FileHeader {
    ident: FileHeaderIdent,
    r#type: HeaderType,
    machine: HeaderMachine,
    //    version: HeaderVersion,
}

impl fmt::Display for FileHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "----------------   HEADER   ---\n{}\n{}\n{}\n",
            self.ident, self.r#type, self.machine,
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
pub fn parse_file_header(data: &[u8]) -> Result<FileHeader, error::Error> {
    let (_rest, mut val) = FileHeader::from_bytes((data.as_ref(), 0)).unwrap();
    Ok(val)
}
*/

pub fn parse_bytes_header_ident(data: &[u8]) -> Result<FileHeaderIdent, error::Error> {
    match FileHeaderIdent::from_bytes((data, 0)) {
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

pub fn parse_bytes(data: &[u8]) -> Result<ElfFile, error::Error> {
    debug!("len: {}", data.len());
    match ElfFile::from_bytes((data, 0)) {
        Ok((_rest, val)) => Ok(val),
        Err(deku_err) => {
            /*
            let ident = match parse_bytes_header_ident(data) {
                Ok(val) => val,
                Err(err) => {
                    Err(err)
                  /*
                    let error_affected = String::from("unknown");
                    let error_suggestion = String::from("verify the file format");
                    let error_type = error::ErrorType::Deku(deku_err);
                    let err = error::Error {
                        error_type,
                        affected: error_affected,
                        suggestion: error_suggestion,
                    };
                    Err(err)
                  */
                }
            };
            */
            println!("{}", deku_err);
            error!("{}", deku_err);
            warn!("Incomplete ELF file. Trying to parse elf header ident ...");
            let ident = parse_bytes_header_ident(data)?;

            //let rest_type;
            let mut header_machine: crate::elf::HeaderMachine = HeaderMachine::None;
            let mut header_type: crate::elf::HeaderType = HeaderType::None;
            // match HeaderType::from_bytes((&data[16..][..18], 0)) {
            if data.len() >= 20 {
                debug!("Bytes HeaderMachine - {:?}", &data[18..][..2]);
                // header_type = match HeaderType::from_bytes((&data[16..][..2], 0)) {
                header_machine = match HeaderMachine::from_bytes((&data[18..][..2], 0)) {
                    Ok((_rest_type, val_type)) => {
                        // header_type = val_type;
                        val_type
                    }
                    Err(type_err) => {
                        println!("{}", type_err);
                        error!("{}", type_err);
                        HeaderMachine::None
                    }
                };
            }
            if data.len() >= 18 {
                println!("Bytes HeaderType - {:?}", &data[16..][..2]);
                header_type = match HeaderType::from_bytes((&data[16..][..2], 0)) {
                    Ok((_rest_type, val_type)) => {
                        // header_type = val_type;
                        val_type
                    }
                    Err(type_err) => {
                        println!("{}", type_err);
                        error!("{}", type_err);
                        HeaderType::None
                    }
                };
            }

            let header = FileHeader {
                ident,
                r#type: header_type,
                machine: header_machine,
                //                version: header_version,
            };
            let elffile = ElfFile {
                file_header: header,
            };
            Ok(elffile)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bytes() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00, 0x3E, 0x00,
        ];
        let val = parse_bytes(&data).unwrap();

        let header_ident_magic = HeaderIdentMagic {
            ei_mag0: 0x7F,
            ei_mag1: 0x45,
            ei_mag2: 0x4C,
            ei_mag3: 0x46,
        };
        let header_ident_class = HeaderIdentClass::Bit32;
        let header_ident_endian = HeaderIdentEndian::Little;
        let header_ident_version = HeaderIdentVersion { ei_version: 0x01 };
        let header_ident_osabi = HeaderIdentOsAbi::Linux;
        let header_ident_abiversion = HeaderIdentAbiVersion {
            ei_abiversion: 0x01,
        };
        let header_ident_padding = HeaderIdentPadding {
            ei_pad: vec![0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17],
        };
        let ident = FileHeaderIdent {
            magic: header_ident_magic,
            class: header_ident_class,
            endian: header_ident_endian,
            version: header_ident_version,
            osabi: header_ident_osabi,
            abiversion: header_ident_abiversion,
            padding: header_ident_padding,
        };
        let header_type = HeaderType::Dyn;
        let header_machine = HeaderMachine::X86_64;
        let header_version = HeaderVersion {
            ei_version: vec![0x00, 0x01, 0x00, 0x00],
        };
        let header = FileHeader {
            ident: ident,
            r#type: header_type,
            machine: header_machine,
            //            version: header_version,
        };
        let expected = ElfFile {
            file_header: header,
        };
        println!("{}", &val);
        assert_eq!(expected, val);

        let data_out = val.to_bytes().unwrap();
        assert_eq!(data, data_out);
    }

    #[test]
    fn test_parse_bytes_header_ident_direct() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x06, 0x07,
        ];
        let val = parse_bytes_header_ident(&data).unwrap();

        let header_ident_magic = HeaderIdentMagic {
            ei_mag0: 0x7F,
            ei_mag1: 0x45,
            ei_mag2: 0x4C,
            ei_mag3: 0x46,
        };
        let header_ident_class = HeaderIdentClass::Bit32;
        let header_ident_endian = HeaderIdentEndian::Little;
        let header_ident_version = HeaderIdentVersion { ei_version: 0x01 };
        let header_ident_osabi = HeaderIdentOsAbi::Linux;
        let header_ident_abiversion = HeaderIdentAbiVersion {
            ei_abiversion: 0x01,
        };
        let header_ident_padding = HeaderIdentPadding {
            ei_pad: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
        };
        let ident = FileHeaderIdent {
            magic: header_ident_magic,
            class: header_ident_class,
            endian: header_ident_endian,
            version: header_ident_version,
            osabi: header_ident_osabi,
            abiversion: header_ident_abiversion,
            padding: header_ident_padding,
        };

        assert_eq!(ident, val);

        let data_out = val.to_bytes().unwrap();
        assert_eq!(data, data_out);
    }

    #[test]
    fn test_parse_bytes_header_ident_only() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x06, 0x07,
        ];
        let val = parse_bytes(&data).unwrap();

        let header_ident_magic = HeaderIdentMagic {
            ei_mag0: 0x7F,
            ei_mag1: 0x45,
            ei_mag2: 0x4C,
            ei_mag3: 0x46,
        };
        let header_ident_class = HeaderIdentClass::Bit32;
        let header_ident_endian = HeaderIdentEndian::Little;
        let header_ident_version = HeaderIdentVersion { ei_version: 0x01 };
        let header_ident_osabi = HeaderIdentOsAbi::Linux;
        let header_ident_abiversion = HeaderIdentAbiVersion {
            ei_abiversion: 0x01,
        };
        let header_ident_padding = HeaderIdentPadding {
            ei_pad: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
        };
        let ident = FileHeaderIdent {
            magic: header_ident_magic,
            class: header_ident_class,
            endian: header_ident_endian,
            version: header_ident_version,
            osabi: header_ident_osabi,
            abiversion: header_ident_abiversion,
            padding: header_ident_padding,
        };
        let header_type = HeaderType::None;
        let header_machine = HeaderMachine::None;
        let header_version = HeaderVersion {
            ei_version: vec![0x01, 0x01, 0x01, 0x01],
        };
        let header = FileHeader {
            ident: ident,
            r#type: header_type,
            machine: header_machine,
            //            version: header_version,
        };
        let expected = ElfFile {
            file_header: header,
        };
        assert_eq!(expected, val);
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
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17,
        ];
        println!("len orig: {}", data.len());
        let val = parse_bytes(&data).unwrap();
        let val_type = val.file_header.r#type;
        let expected = HeaderType::None;
        assert_eq!(expected, val_type);
    }

    #[test]
    fn test_parse_header_invalid_machine() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00, 0x00, 0x00,
        ];
        println!("len orig: {}", data.len());
        let val = parse_bytes(&data).unwrap();

        let header_ident_magic = HeaderIdentMagic {
            ei_mag0: 0x7F,
            ei_mag1: 0x45,
            ei_mag2: 0x4C,
            ei_mag3: 0x46,
        };
        let header_ident_class = HeaderIdentClass::Bit32;
        let header_ident_endian = HeaderIdentEndian::Little;
        let header_ident_version = HeaderIdentVersion { ei_version: 0x01 };
        let header_ident_osabi = HeaderIdentOsAbi::Linux;
        let header_ident_abiversion = HeaderIdentAbiVersion {
            ei_abiversion: 0x01,
        };
        let header_ident_padding = HeaderIdentPadding {
            ei_pad: vec![0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17],
        };
        let ident = FileHeaderIdent {
            magic: header_ident_magic,
            class: header_ident_class,
            endian: header_ident_endian,
            version: header_ident_version,
            osabi: header_ident_osabi,
            abiversion: header_ident_abiversion,
            padding: header_ident_padding,
        };
        let header_type = HeaderType::Dyn;
        let header_machine = HeaderMachine::None;
        let header_version = HeaderVersion {
            ei_version: vec![0x00, 0x01, 0x00, 0x00],
        };
        let header = FileHeader {
            ident: ident,
            r#type: header_type,
            machine: header_machine,
            //            version: header_version,
        };
        let expected = ElfFile {
            file_header: header,
        };
        println!("{}", &val);
        assert_eq!(expected, val);

        let data_out = val.to_bytes().unwrap();
        assert_eq!(data, data_out);
    }

    #[test]
    fn test_parse_bytes_incomplete_machine() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00,
        ];
        let val = parse_bytes(&data).unwrap();
        let val_type = val.file_header.r#type;
        let val_machine = val.file_header.machine;
        let expected_type = HeaderType::Dyn;
        let expected_machine = HeaderMachine::None;
        assert_eq!(expected_type, val_type);
        assert_eq!(expected_machine, val_machine);
    }

    #[test]
    fn test_parse_bytes_machine_direct() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x03, 0x00, 0x3E, 0x00,
        ];
        match ElfFile::from_bytes((&data, 0)) {
            Ok((rest, val)) => {
                println!("{:?}", rest);
                println!("{}", val);
                let val_type = val.file_header.r#type;
                let val_machine = val.file_header.machine;
                let expected_type = HeaderType::Dyn;
                let expected_machine = HeaderMachine::X86_64;
                assert_eq!(expected_type, val_type);
                assert_eq!(expected_machine, val_machine);
            }
            Err(err) => {
                /*
                let ident = match parse_bytes_header_ident(data) {
                    Ok(val) => val,
                    Err(err) => {
                        Err(err)
                      /*
                        let error_affected = String::from("unknown");
                        let error_suggestion = String::from("verify the file format");
                        let error_type = error::ErrorType::Deku(deku_err);
                        let err = error::Error {
                            error_type,
                            affected: error_affected,
                            suggestion: error_suggestion,
                        };
                        Err(err)
                      */
                    }
                };
                */
                println!("{:?}", err);
                error!("{}", err);
                warn!("Incomplete ELF file. Trying to parse elf header ident ...");
                assert!(false);
            }
        }
    }

    #[test]
    fn test_parse_header_type() {
        let data: Vec<u8> = vec![0x01, 0x00];
        let (_rest, val) = HeaderType::from_bytes((&data, 0)).unwrap();
        let expected = HeaderType::Rel;
        println!("{:?}", val);
        assert_eq!(expected, val);
    }

    #[test]
    fn test_parse_header_machine() {
        let data: Vec<u8> = vec![0x3E, 0x00];
        let (_rest, val) = HeaderMachine::from_bytes((&data, 0)).unwrap();
        let expected = HeaderMachine::X86_64;
        println!("{:?}", val);
        assert_eq!(expected, val);
    }
}
