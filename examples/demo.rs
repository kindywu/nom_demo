use std::error::Error;

use nom::{bytes::complete::tag, IResult};

fn parse_input(input: &str) -> IResult<&str, &str> {
    tag("abc")(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (leftover, output) = parse_input("abc123")?;
    assert_eq!(leftover, "123");
    assert_eq!(output, "abc");

    assert!(parse_input("efg").is_err());
    Ok(())
}
