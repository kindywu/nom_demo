use std::error::Error;

use nom::{
    bytes::complete::tag,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn main() -> Result<(), Box<dyn Error>> {
    let (_, parsed) = parse_coordinate("(3, 5)")?;
    assert_eq!(parsed, Coordinate { x: 3, y: 5 });
    let (_, parsed) = parse_coordinate("(3,5)")?;
    assert_eq!(parsed, Coordinate { x: 3, y: 5 });

    assert!(parse_coordinate("3,5)").is_err());
    Ok(())
}

#[derive(Debug, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}
use nom::character::complete::i32;

fn parse_i32_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(i32, tag(", ").or(tag(",")), i32)(input)
}

fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    let (remaining, (x, y)) = delimited(tag("("), parse_i32_pair, tag(")"))(input)?;

    Ok((remaining, Coordinate { x, y }))
}
