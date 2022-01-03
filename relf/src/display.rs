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
