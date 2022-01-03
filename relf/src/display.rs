use std::fmt;
extern crate hexplay;
use hexplay::HexViewBuilder;

pub fn print_field(f: &mut fmt::Formatter, name: &str, value: &str, data: &[u8]) -> fmt::Result {
    write!(f, "{:<14} | {:>20} | {}", name, value, to_hex(data),)
}

fn to_hex(data: &[u8]) -> String {
    let view = HexViewBuilder::new(data)
        .address_offset(20)
        .replacement_character('.')
        .row_width(16)
        .finish();

    let hex = format!("{}", view);

    (&hex[8..]).to_string()
}
