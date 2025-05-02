use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::{IResult, Parser};

fn parse_base(input: &str) -> IResult<&str, &str> {
    alt((
        tag_no_case("a"),
        tag_no_case("t"),
        tag_no_case("c"),
        tag_no_case("g"),
    ))
    .parse(input)
}

fn parse_pair(input: &str) -> IResult<&str, (&str, &str)> {
    // the many_m_n combinator might also be appropriate here.
    (parse_base, parse_base).parse(input)
}

fn main() -> anyhow::Result<()> {
    let (remaining, parsed) = parse_pair("aTcG")?;
    assert_eq!(parsed, ("a", "T"));
    assert_eq!(remaining, "cG");

    assert!(parse_pair("Dct").is_err());
    Ok(())
}
