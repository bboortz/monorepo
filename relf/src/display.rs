use std::fmt;
extern crate hexplay;
use hexplay::HexViewBuilder;

static mut OFFSET: usize = 0;

pub fn print_field(f: &mut fmt::Formatter, name: &str, value: &str, data: &[u8]) -> fmt::Result {
    write!(f, "{:<14} | {:>28} | {}", name, value, to_hex(data),)
}

fn to_hex(data: &[u8]) -> String {
    let offset: usize;
    let data_len = data.len();
    unsafe {
        offset = OFFSET;
    }
    let view = HexViewBuilder::new(data)
        .address_offset(offset)
        .replacement_character('.')
        .row_width(16)
        .finish();
    unsafe {
        OFFSET += data_len;
    }

    format!("{}", view)

    /*
    let hex = format!("{}", view);
    (&hex[8..]).to_string()
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_field() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x06, 0x07, 0x03, 0x3E,
        ];
        let expected = String::from("[127, 69, 76, 70, 1, 1, 1, 3, 1, 1, 2, 3, 4, 5, 6, 7, 3, 62]");
        //let result = print_field(f, "KEY", "VALUE", &data);
        let result = format!("{:?}", data);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tohex() {
        let data: Vec<u8> = vec![
            0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01, 0x03, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x06, 0x07, 0x03, 0x3E,
        ];
        let expected = String::from("00000000  7F 45 4C 46 01 01 01 03 01 01 02 03 04 05 06 07  | ⌂ELF☺☺☺♥☺☺☻♥♦♣♠• |\n00000010  03 3E                                            | ♥>               |");
        let result = to_hex(&data);
        assert_eq!(expected, result);
    }
}
