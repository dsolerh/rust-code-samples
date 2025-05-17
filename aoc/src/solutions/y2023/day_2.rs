use crate::solutions::Solver;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        char,
        complete::{i32 as parse_i32, u8 as parse_u8},
    },
    multi::separated_list0,
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Solver for Solution {
    fn solve_part1(&self, data: Vec<String>) -> String {
        let value: i32 = data
            .iter()
            .filter_map(|s| parse_game(s.as_str()).ok())
            .filter(|(_, game)| {
                game.cubes
                    .iter()
                    .all(|cube| cube.red <= 12 && cube.green <= 13 && cube.blue <= 14)
            })
            .map(|(_, game)| game.id)
            .sum();
        format!("{value}")
    }

    fn solve_part2(&self, data: Vec<String>) -> String {
        let value: i32 = data
            .iter()
            .filter_map(|s| parse_game(s.as_str()).ok())
            .map(|(_, game)| {
                let min_cube = game
                    .cubes
                    .into_iter()
                    .reduce(|mut acc, cube| {
                        if cube.red > acc.red {
                            acc.red = cube.red
                        }
                        if cube.green > acc.green {
                            acc.green = cube.green
                        }
                        if cube.blue > acc.blue {
                            acc.blue = cube.blue
                        }

                        acc
                    })
                    .unwrap();

                min_cube.red as i32 * min_cube.green as i32 * min_cube.blue as i32
            })
            .sum();
        format!("{value}")
    }
}

#[derive(Debug, PartialEq)]
struct Cube {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: i32,
    cubes: Vec<Cube>,
}

fn parse_game(s: &str) -> IResult<&str, Game> {
    let (left, _) = tag("Game ")(s)?;
    let parse_colors = separated_list0(tag("; "), parse_cube);
    let (_, (id, cubes)) = separated_pair(parse_i32, tag(": "), parse_colors).parse(left)?;

    Ok(("", Game { id, cubes }))
}

fn parse_cube(s: &str) -> IResult<&str, Cube> {
    let parse_single_color = separated_pair(
        parse_u8,
        char(' '),
        alt((tag("green"), tag("blue"), tag("red"))),
    );
    let mut parse_full_color = separated_list0(tag(", "), parse_single_color);
    let (left, res) = parse_full_color.parse(s)?;
    let mut cube = Cube {
        red: 0,
        green: 0,
        blue: 0,
    };
    for (val, c) in res {
        match c {
            "red" => cube.red = val,
            "blue" => cube.blue = val,
            "green" => cube.green = val,
            _ => unreachable!(),
        }
    }
    Ok((left, cube))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1 red, 5 blue, 1 green", Ok(("",Cube{red:1,green:1,blue:5})))]
    #[case("12 red, 2 blue", Ok(("",Cube{red:12,green:0,blue:2})))]
    fn test_parse_cube(#[case] input: String, #[case] expected: IResult<&str, Cube>) {
        let res = parse_cube(input.as_str());
        assert_eq!(res, expected);
    }

    #[rstest]
    #[case(
        "Game 1: 1 red, 5 blue, 1 green; 16 blue, 3 red; 6 blue, 5 red; 4 red, 7 blue, 1 green",
        Ok(("",Game{
            id:1,
            cubes:vec![
                Cube{red:1,green:1,blue:5},
                Cube{red:3,green:0,blue:16},
                Cube{red:5,green:0,blue:6},
                Cube{red:4,green:1,blue:7},
            ]}))
    )]
    #[case(
        "Game 1: 1 red, 5 blue, 1 green; 6 blue, 5 red",
        Ok(("",Game{
            id:1,
            cubes:vec![
                Cube{red:1,green:1,blue:5},
                Cube{red:5,green:0,blue:6},
            ]}))
    )]
    fn test_parse_game(#[case] input: &str, #[case] expected: IResult<&str, Game>) {
        let res = parse_game(input);
        assert_eq!(res, expected);
    }
}
