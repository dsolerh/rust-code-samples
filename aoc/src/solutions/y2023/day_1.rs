use crate::solutions::Solver;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Solver for Solution {
    fn solve_part1(&self, data: Vec<String>) -> String {
        let value: i32 = data.into_iter().map(parse_num).sum();
        format!("{value}")
    }

    fn solve_part2(&self, data: Vec<String>) -> String {
        let value: i32 = data.into_iter().map(parse_num_with_literals).sum();
        format!("{value}")
    }
}

const ZERO: u8 = 48;

#[inline]
fn u8_to_num(v: u8) -> i32 {
    (v - ZERO) as i32
}

/// will parse the number according to some special rules
fn parse_num(s: String) -> i32 {
    let mut nums = vec![];
    for &b in s.as_bytes() {
        if b.is_ascii_digit() {
            nums.push(b);
        }
    }
    let first = u8_to_num(*nums.first().unwrap());
    let last = u8_to_num(*nums.last().unwrap());
    10 * first + last
}

const LITERALS: [(&str, i32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn parse_num_with_literals(s: String) -> i32 {
    let mut nums = vec![];

    for i in 0..s.len() {
        // Check for numeric digit
        if let Some(c) = s[i..].chars().next() {
            if c.is_ascii_digit() {
                nums.push(c.to_digit(10).unwrap() as i32);
                continue;
            }
        }

        // Check for spelled-out numbers
        for (word, value) in &LITERALS {
            if s[i..].starts_with(word) {
                nums.push(*value);
                break;
            }
        }
    }

    let first = *nums.first().unwrap();
    let last = *nums.last().unwrap();
    10 * first + last
}

#[cfg(test)]
mod tests {
    use super::{parse_num, parse_num_with_literals, u8_to_num};
    use rstest::rstest;

    #[test]
    fn parse_num_example1() {
        let num = parse_num("1abc2".to_string());
        assert_eq!(num, 12);
    }

    #[test]
    fn covert_u8_num() {
        let bytes = "0123456789".as_bytes();
        for (i, &b) in bytes.into_iter().enumerate() {
            let num = u8_to_num(b);
            assert_eq!(num, i as i32, "ascii: {} -> num: {}", b, num);
        }
    }

    #[rstest]
    #[case("9", 99)] // Single digit
    #[case("nine", 99)] // Single spelled number
    #[case("eighthree", 83)] // Simple overlap
    #[case("sevenine", 79)] // "sevenine" overlap
    #[case("one2dsa4", 14)] // digits and literals
    #[case("2dsa4twos", 22)]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn parse_num_with_literals_example1(#[case] input: String, #[case] expected: i32) {
        let got = parse_num_with_literals(input);
        assert_eq!(got, expected)
    }
}
