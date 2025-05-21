use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::bytes::tag;
use nom::character::complete::i32 as parse_i32;
use nom::sequence::separated_pair;
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

#[allow(dead_code)]
fn parse_integer_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(parse_i32, tag(", "), parse_i32).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("23, 32", (23,32))]
    fn test_parse_integer_pair(#[case] input: String, #[case] expected: (i32, i32)) {
        let (_, result) = parse_integer_pair(input.as_str()).unwrap();
        assert_eq!(result, expected);
    }
}
