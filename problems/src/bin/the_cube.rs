use std::str::FromStr;

fn main() {
    let countdown = calculate_cube_countdown(GAMES);

    println!("the cube countdown is: {countdown}");

    let power = calculate_cube_power(GAMES);

    println!("the cube power is: {power}");
}

fn calculate_cube_countdown(data: &str) -> i32 {
    data.lines()
        .map(str::trim)
        .map(Game::from_str)
        // .inspect(|g|println!("{:?}",g))
        .filter_map(|r| r.ok())
        // .inspect(|g|println!("{:?}",g))
        .filter(|game| {
            game.cubes
                .iter()
                .all(|cubes| cubes.red <= 12 && cubes.green <= 13 && cubes.blue <= 14)
        })
        .map(|game| game.id)
        .sum()
}

fn calculate_cube_power(data: &str) -> i32 {
    data.lines()
        .map(str::trim)
        .map(Game::from_str)
        // .inspect(|g|println!("{:?}",g))
        .filter_map(|r| r.ok())
        // .inspect(|g|println!("{:?}",g))
        .map(|game| {
            let min_cube = game.cubes.into_iter().reduce(|mut acc, cube| {
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
            }).unwrap_or_default();
            
            min_cube.red as i32 * min_cube.green as i32 * min_cube.blue as i32
        })
        .sum()
}

#[derive(Debug, PartialEq)]
enum ParseGameError {
    NoGameId,
    NoGameCubes,
    InvalidGameInfo(String),
    InvalidGameId(String),
    InvalidCubesInfo(ParseCubeError),
}

#[derive(Debug, PartialEq)]
struct ParseCubeError(String);

#[derive(Debug, PartialEq, Default)]
struct Cubes {
    red: u8,
    green: u8,
    blue: u8,
}

impl FromStr for Cubes {
    type Err = ParseCubeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 1 red, 5 blue, 1 gree
        let mut cube = Self::default();
        for seg in s.split(", ") {
            let mut parts = seg.split(' ');
            let val = parts.next().ok_or(ParseCubeError(
                format!("invalid color value in line: {} part: {}", s, seg).to_string(),
            ))?;
            let val = val.parse::<u8>().map_err(|_| {
                ParseCubeError(
                    format!("invalid color value in line: {} part: {}", s, seg).to_string(),
                )
            })?;
            let color = parts.next().ok_or(ParseCubeError(
                format!("invalid color in line: {} part: {}", s, seg).to_string(),
            ))?;

            match color {
                "red" => cube.red = val,
                "green" => cube.green = val,
                "blue" => cube.blue = val,
                _ => {
                    return Err(ParseCubeError(
                        format!("invalid color in line: {} part: {}", s, seg).to_string(),
                    ))
                }
            }
        }

        Ok(cube)
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: i32,
    cubes: Vec<Cubes>,
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Game 1: 1 red, 5 blue, 1 green; 16 blue, 3 red; 6 blue, 5 red; 4 red, 7 blue, 1 green
        let mut s = s.split(": ");

        let game_info = s.next().ok_or(ParseGameError::NoGameId)?;
        let cubes_info = s.next().ok_or(ParseGameError::NoGameCubes)?;

        let id = game_info
            .split(" ")
            .nth(1)
            .ok_or(ParseGameError::InvalidGameInfo(game_info.to_string()))?;
        let id = id
            .parse::<i32>()
            .map_err(|_| ParseGameError::InvalidGameId(id.to_string()))?;

        let cubes = cubes_info
            .split("; ")
            .map(Cubes::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ParseGameError::InvalidCubesInfo(e))?;

        Ok(Game { id, cubes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubes_from_str() {
        let cubes = "1 red, 5 blue, 1 green".parse::<Cubes>();

        assert_eq!(
            cubes,
            Ok(Cubes {
                red: 1,
                blue: 5,
                green: 1
            })
        );

        let cubes = "1 red, 1 green".parse::<Cubes>();

        assert_eq!(
            cubes,
            Ok(Cubes {
                red: 1,
                blue: 0,
                green: 1
            })
        );

        let cubes = "1 green".parse::<Cubes>();

        assert_eq!(
            cubes,
            Ok(Cubes {
                red: 0,
                blue: 0,
                green: 1
            })
        );

        let cubes = "1s red".parse::<Cubes>();

        assert_eq!(
            cubes,
            Err(ParseCubeError(
                "invalid color value in line: 1s red part: 1s red".to_string()
            ))
        );

        let cubes = "1 rd".parse::<Cubes>();

        assert_eq!(
            cubes,
            Err(ParseCubeError(
                "invalid color in line: 1 rd part: 1 rd".to_string()
            ))
        )
    }

    #[test]
    fn test_game_from_str() {
        let game =
            "Game 1: 1 red, 5 blue, 1 green; 16 blue, 3 red; 6 blue, 5 red; 4 red, 7 blue, 1 green"
                .parse::<Game>();

        assert_eq!(
            game,
            Ok(Game {
                id: 1,
                cubes: vec![
                    Cubes {
                        red: 1,
                        blue: 5,
                        green: 1
                    },
                    Cubes {
                        blue: 16,
                        red: 3,
                        green: 0
                    },
                    Cubes {
                        blue: 6,
                        red: 5,
                        green: 0
                    },
                    Cubes {
                        red: 4,
                        blue: 7,
                        green: 1
                    }
                ]
            })
        )
    }

    #[test]
    fn test_calculate_cube_countdown() {
        let total = calculate_cube_countdown(TEST_DATA);

        assert_eq!(total, 8)
    }

    #[test]
    fn test_calculate_cube_power() {
        let power = calculate_cube_power(TEST_DATA);

        assert_eq!(power, 2286)
    }

    const TEST_DATA: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
}

const GAMES: &str = r"Game 1: 1 red, 5 blue, 1 green; 16 blue, 3 red; 6 blue, 5 red; 4 red, 7 blue, 1 green
Game 2: 4 blue; 4 red, 3 blue, 1 green; 4 red, 9 blue, 2 green; 5 blue, 7 green, 4 red
Game 3: 10 blue; 7 blue, 1 green; 19 blue, 1 green, 9 red
Game 4: 2 green; 14 blue, 14 red, 4 green; 12 red, 11 green, 13 blue; 5 green, 9 red, 4 blue; 9 red, 7 green, 12 blue; 2 green, 3 blue, 8 red
Game 5: 3 blue, 4 red; 12 red, 2 green, 15 blue; 1 red, 10 blue, 1 green
Game 6: 1 blue, 7 red; 3 green, 5 red, 1 blue; 1 green, 7 red; 6 red, 1 blue, 4 green; 1 green, 8 red, 1 blue; 2 green, 4 red, 1 blue
Game 7: 11 green, 10 blue, 2 red; 1 green, 12 blue, 2 red; 9 green, 14 blue; 1 red, 19 blue, 15 green
Game 8: 4 green, 2 red, 14 blue; 9 green, 1 red, 15 blue; 2 green, 9 red, 8 blue; 11 green, 7 red, 8 blue; 9 red, 7 green, 6 blue
Game 9: 4 blue, 1 green, 2 red; 1 blue, 3 red; 1 red, 3 blue, 3 green
Game 10: 4 red, 3 green, 6 blue; 2 green, 15 blue, 6 red; 3 green, 2 blue; 2 red, 1 green; 11 blue, 7 red, 4 green; 2 blue, 2 red, 4 green
Game 11: 10 red, 1 green, 7 blue; 5 red, 2 green, 7 blue; 2 green, 4 blue; 1 green, 10 red, 10 blue; 8 blue, 4 red
Game 12: 6 green, 2 blue, 7 red; 3 green, 2 blue, 3 red; 6 red, 1 blue, 9 green; 9 green, 13 red, 5 blue; 6 green, 4 blue, 8 red
Game 13: 10 green, 4 red, 6 blue; 19 red, 6 green, 7 blue; 6 blue, 5 red, 8 green
Game 14: 4 blue, 2 green; 19 blue; 6 red, 17 blue; 10 blue, 7 red; 1 green, 2 blue, 7 red
Game 15: 4 green, 12 blue, 15 red; 10 blue, 18 green, 13 red; 20 blue, 6 green, 10 red; 20 red, 12 blue, 13 green; 12 blue, 17 green, 10 red; 1 red, 3 blue, 7 green
Game 16: 1 blue, 6 red, 5 green; 3 red, 3 green; 5 green, 1 red; 2 red, 1 blue, 6 green; 1 blue, 1 red, 6 green; 1 blue, 6 green
Game 17: 4 red, 3 blue, 3 green; 8 blue, 8 green; 5 red, 3 green, 9 blue; 9 green, 12 blue, 13 red; 1 green, 1 blue, 5 red; 7 green, 6 red
Game 18: 2 green, 11 blue, 6 red; 2 green, 11 red, 2 blue; 7 red, 4 blue, 9 green; 18 blue, 6 red, 1 green
Game 19: 4 red, 7 green, 17 blue; 5 green, 6 red, 4 blue; 4 blue, 4 red, 1 green
Game 20: 2 blue, 5 green, 9 red; 4 green, 8 red, 10 blue; 7 blue, 9 red, 1 green; 1 green, 10 blue, 9 red; 1 green, 8 red; 8 blue, 8 red, 1 green
Game 21: 1 blue, 14 red; 1 green, 2 red; 9 red, 1 blue, 1 green
Game 22: 7 green, 9 red, 4 blue; 9 red, 7 green, 9 blue; 8 green, 14 red; 5 blue; 10 red, 1 blue, 1 green; 8 green
Game 23: 2 red, 12 green, 5 blue; 3 red, 5 blue, 3 green; 1 red, 9 green, 1 blue; 8 green, 6 blue; 13 green
Game 24: 8 red, 7 green, 1 blue; 1 red, 6 green, 7 blue; 1 green, 3 red
Game 25: 4 green, 2 red; 1 red, 2 green, 8 blue; 1 green; 8 blue
Game 26: 1 green, 4 blue, 17 red; 15 red, 3 green, 3 blue; 2 blue, 2 red; 18 red, 2 green, 11 blue; 6 red, 7 blue; 10 blue, 1 green, 4 red
Game 27: 2 red, 5 blue, 1 green; 14 green, 2 red, 6 blue; 1 red, 4 blue, 14 green
Game 28: 3 red, 5 green, 2 blue; 2 red, 3 green, 4 blue; 1 red, 9 green, 3 blue; 13 green, 4 red, 4 blue
Game 29: 18 red, 11 green; 4 blue, 18 red, 9 green; 16 red, 2 green, 4 blue; 2 red, 3 blue, 12 green; 1 green, 18 red; 2 blue, 15 green, 1 red
Game 30: 10 red, 3 blue, 1 green; 6 red, 1 blue, 3 green; 2 green, 2 blue, 10 red; 6 green; 3 blue, 15 red
Game 31: 1 blue, 7 green, 2 red; 12 red, 8 green, 4 blue; 2 green, 2 blue, 5 red; 2 blue, 3 green, 12 red; 7 red, 5 green, 4 blue; 7 red, 1 blue
Game 32: 4 blue, 5 red, 11 green; 20 red, 8 green, 1 blue; 10 red, 7 green, 1 blue; 1 blue, 7 red, 2 green; 1 red, 19 green, 3 blue
Game 33: 9 red; 9 red, 6 green, 7 blue; 5 red, 7 blue, 2 green
Game 34: 5 green, 5 red, 3 blue; 8 green, 6 blue, 16 red; 12 blue, 8 red, 8 green; 1 blue, 10 red, 3 green; 1 green, 13 blue, 18 red; 4 blue, 5 green, 8 red
Game 35: 15 green, 4 red, 8 blue; 7 red, 1 green, 14 blue; 12 green, 16 blue, 2 red
Game 36: 3 blue, 3 green, 2 red; 7 red, 8 blue; 11 blue, 9 red; 4 red, 13 blue, 1 green
Game 37: 4 red, 11 blue, 8 green; 6 green, 4 blue, 14 red; 5 blue, 7 green, 13 red; 6 red, 2 green, 5 blue; 4 red, 3 blue, 1 green; 6 red, 4 green, 6 blue
Game 38: 10 green, 5 blue, 1 red; 3 red, 6 blue, 3 green; 9 green, 9 blue, 3 red; 9 blue, 1 red, 6 green
Game 39: 3 blue, 16 red; 10 red, 4 green, 2 blue; 2 blue, 13 red, 1 green; 2 blue, 11 red, 2 green; 3 green, 13 red
Game 40: 2 blue, 3 red, 2 green; 2 green, 2 blue, 6 red; 1 green, 9 red
Game 41: 1 blue, 12 red; 8 blue, 1 red, 5 green; 1 green, 7 blue, 13 red; 8 red, 7 blue, 7 green; 4 green, 17 red, 9 blue; 2 green, 8 blue
Game 42: 2 green, 6 red, 1 blue; 3 red, 2 green; 9 red, 1 green; 2 red, 2 green
Game 43: 10 blue, 9 red; 14 blue, 4 green; 5 red, 3 green, 9 blue; 5 blue, 8 green, 1 red
Game 44: 3 blue, 10 green, 1 red; 1 blue, 13 red, 3 green; 1 blue, 5 green, 16 red
Game 45: 1 red, 1 green, 3 blue; 2 green, 1 red, 5 blue; 1 red, 2 blue, 1 green; 1 blue; 1 green, 5 blue; 1 blue
Game 46: 8 green, 8 blue, 4 red; 10 green, 4 red, 7 blue; 2 red, 3 green, 14 blue
Game 47: 3 green, 3 red; 5 green, 2 blue, 6 red; 3 blue, 5 red, 15 green; 2 green, 2 blue, 2 red
Game 48: 11 blue, 12 green, 3 red; 8 blue, 3 red, 3 green; 1 green, 6 blue, 2 red
Game 49: 3 blue, 17 green, 1 red; 4 red, 16 blue, 17 green; 1 green, 3 red, 5 blue; 14 blue, 1 red, 12 green
Game 50: 2 blue, 5 red, 6 green; 8 blue, 11 green, 5 red; 2 green, 2 red, 6 blue
Game 51: 1 green, 2 red; 4 green; 1 blue, 10 green
Game 52: 8 blue, 9 red, 4 green; 2 green, 8 blue, 2 red; 1 red, 2 green, 1 blue; 2 blue, 8 green, 8 red; 4 red, 1 green, 9 blue; 11 blue, 4 green, 8 red
Game 53: 1 green, 2 red; 3 blue, 1 green, 9 red; 5 blue, 11 red; 4 blue, 6 red, 1 green; 5 blue, 10 red; 5 blue, 5 red, 1 green
Game 54: 1 blue, 8 green; 9 green, 1 red, 11 blue; 16 green, 8 blue; 5 green
Game 55: 7 blue, 2 red, 1 green; 16 green, 19 blue, 5 red; 9 green, 3 blue, 7 red; 8 blue, 2 green, 4 red; 8 green, 15 blue, 5 red
Game 56: 9 blue, 1 red, 4 green; 12 green, 12 blue; 1 green, 1 red, 5 blue
Game 57: 1 green, 10 blue; 1 red, 9 blue; 10 blue, 1 red, 3 green
Game 58: 6 red, 15 blue, 3 green; 13 blue, 5 red; 10 blue, 2 red; 5 red, 1 green, 14 blue
Game 59: 7 red, 1 blue, 9 green; 4 green, 12 red, 2 blue; 6 green, 20 red, 1 blue; 4 blue, 9 red, 2 green; 8 red, 4 blue, 2 green
Game 60: 11 red, 8 blue, 1 green; 18 green, 11 blue; 16 red, 10 blue, 7 green; 6 blue, 8 red; 7 red, 15 green, 4 blue
Game 61: 1 blue, 1 green, 8 red; 3 red, 7 blue; 4 blue, 10 red; 1 green, 5 red, 8 blue; 10 red, 7 blue
Game 62: 12 blue, 1 red, 1 green; 2 green, 1 red, 7 blue; 3 green, 18 blue; 11 blue, 4 green
Game 63: 4 green, 4 red, 8 blue; 7 red, 5 blue, 5 green; 2 green, 20 blue, 4 red; 1 green, 4 blue, 3 red
Game 64: 2 green, 2 red; 3 green, 2 blue; 12 green, 2 red, 4 blue; 5 red, 9 green, 8 blue; 7 blue, 6 green; 3 green, 5 red
Game 65: 8 red, 2 green, 13 blue; 11 blue; 7 blue, 2 green; 12 blue, 1 green, 9 red
Game 66: 1 blue, 3 red, 19 green; 3 red, 17 blue, 15 green; 9 green, 9 blue
Game 67: 2 green, 7 blue, 1 red; 3 green, 1 red, 7 blue; 1 red, 6 green; 7 blue, 2 red, 10 green; 2 red, 5 green, 4 blue
Game 68: 14 red, 10 green, 8 blue; 11 red, 1 blue, 6 green; 7 red, 7 green; 12 blue, 10 green, 3 red; 6 red, 12 blue, 10 green; 8 green, 14 red, 3 blue
Game 69: 4 green, 8 red; 2 red, 15 green; 5 red, 1 blue, 12 green; 13 red, 6 green; 10 green, 13 red, 1 blue
Game 70: 3 red, 10 blue, 3 green; 8 red, 11 blue, 11 green; 5 red, 13 green
Game 71: 18 green, 3 red, 1 blue; 3 blue, 14 green, 2 red; 6 blue, 20 green, 4 red
Game 72: 2 blue, 1 red; 2 blue, 3 green, 1 red; 4 blue, 2 red, 4 green
Game 73: 11 red, 11 green; 5 green, 1 blue; 8 red, 7 green, 4 blue; 5 blue, 7 red, 12 green
Game 74: 12 red, 12 green, 5 blue; 10 red, 7 blue, 15 green; 6 green, 19 red, 19 blue; 3 red, 7 blue, 16 green; 11 red, 14 green, 16 blue
Game 75: 5 red, 17 green, 8 blue; 10 red, 8 blue, 19 green; 9 blue, 6 red, 18 green; 3 blue, 13 red, 12 green
Game 76: 5 green, 2 red, 8 blue; 3 blue, 14 red, 2 green; 14 red, 1 blue; 3 green, 8 blue, 15 red; 11 red, 1 green; 11 red, 9 blue, 3 green
Game 77: 3 blue, 2 red; 1 blue, 8 green, 11 red; 14 green, 14 red; 3 red, 5 green, 5 blue; 2 green, 16 blue, 3 red; 13 red, 7 green, 5 blue
Game 78: 3 blue, 1 green, 1 red; 5 blue, 1 green, 1 red; 9 blue, 7 red, 1 green; 5 blue, 1 green, 5 red; 10 blue, 3 green, 7 red
Game 79: 19 green, 17 blue, 4 red; 7 green, 7 red, 16 blue; 4 red, 10 green; 13 blue, 17 green, 2 red
Game 80: 9 blue, 3 green; 15 blue, 1 red; 3 blue, 12 green, 2 red; 1 red, 14 green, 13 blue; 1 red, 10 blue, 16 green; 8 blue, 6 green, 2 red
Game 81: 1 green, 3 red, 19 blue; 2 red, 1 green, 9 blue; 1 green, 2 red, 8 blue; 1 red, 1 green, 11 blue; 1 green, 3 red, 11 blue
Game 82: 8 red, 1 blue, 4 green; 9 green, 3 blue, 4 red; 3 green, 3 blue, 18 red
Game 83: 3 red, 13 blue, 16 green; 16 green, 2 blue; 14 green, 12 blue; 8 green, 14 blue, 4 red; 12 green, 4 blue; 20 green, 1 red
Game 84: 4 green, 4 blue, 5 red; 6 red, 6 blue, 8 green; 5 blue, 12 green, 3 red; 5 red, 13 green; 6 blue, 1 green, 5 red
Game 85: 10 green; 7 green, 1 blue; 5 red, 5 blue, 1 green; 2 green, 2 red, 3 blue; 3 red, 10 green, 3 blue; 1 blue, 1 red
Game 86: 3 green, 1 red, 3 blue; 2 red, 2 green; 9 green, 2 blue, 3 red; 3 red, 3 blue, 4 green
Game 87: 6 red, 4 green; 1 red, 3 green, 5 blue; 1 green, 7 blue, 4 red
Game 88: 2 green, 4 red, 3 blue; 5 green, 1 blue; 3 red, 5 green, 2 blue; 1 green, 6 red, 1 blue; 7 red, 2 blue; 17 red, 13 green
Game 89: 4 green, 2 blue, 6 red; 15 red, 7 green, 10 blue; 7 red, 9 blue, 4 green
Game 90: 9 red, 17 blue; 1 green, 9 blue; 5 red, 8 blue; 3 blue, 9 red, 1 green; 17 blue, 1 red
Game 91: 7 green, 3 red, 5 blue; 4 blue, 3 red, 9 green; 9 red, 7 blue, 7 green; 5 red, 6 blue, 3 green; 10 red, 2 green, 6 blue
Game 92: 13 blue, 8 red; 7 green, 1 red, 8 blue; 5 blue, 4 red, 2 green; 9 red, 10 blue
Game 93: 6 green; 1 blue, 16 green, 6 red; 5 green, 1 blue, 5 red; 5 red, 6 green; 16 green, 2 red, 1 blue; 11 green, 2 red
Game 94: 9 blue, 4 green; 12 green, 17 blue; 4 green, 5 blue, 6 red; 2 red, 2 blue, 12 green
Game 95: 5 red, 4 blue, 5 green; 2 blue, 4 green, 4 red; 4 blue, 2 red, 7 green; 1 green, 7 blue, 8 red
Game 96: 7 blue, 6 green, 2 red; 3 green, 1 blue; 7 blue, 3 red, 5 green; 1 green, 5 blue; 6 blue, 2 red; 2 green, 1 red
Game 97: 10 red, 1 green, 1 blue; 4 green, 11 red, 2 blue; 4 red, 1 blue, 4 green
Game 98: 3 green, 4 blue, 7 red; 7 red, 8 green; 7 green, 16 red, 1 blue; 8 green, 2 blue, 4 red; 5 green, 3 blue, 18 red
Game 99: 6 green, 12 red, 1 blue; 5 blue, 1 red, 7 green; 5 green, 7 red, 10 blue; 8 blue, 1 red, 7 green; 17 red, 4 blue, 9 green
Game 100: 6 blue, 10 green; 3 green, 4 blue, 1 red; 7 blue, 1 red, 12 green";
