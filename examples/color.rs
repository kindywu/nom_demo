use std::{error::Error, num::ParseIntError};

use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::Tuple,
    AsChar, IResult,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = "#2F14DF";
    let (_, color) = hex_color(input)?;
    assert_eq!(
        color,
        Color {
            r: 47,
            g: 20,
            b: 223
        }
    );
    Ok(())
}

#[derive(Debug, PartialEq)]
struct Color {
    r: u8,
    b: u8,
    g: u8,
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (r, g, b)) = (hex_primary, hex_primary, hex_primary).parse(input)?;
    Ok((input, Color { r, g, b }))
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex), from_hex)(input)
}

fn from_hex(input: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex(c: char) -> bool {
    c.is_hex_digit()
}
