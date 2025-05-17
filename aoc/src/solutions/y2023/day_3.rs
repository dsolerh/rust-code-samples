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
        let numbers = NumberPos::from_data(&data);
        let symbols = SymbolPos::from_data(&data);

        let value: i32 = numbers
            .into_iter()
            .filter(|number| {
                symbols
                    .iter()
                    .find(|symbol| {
                        symbol.index >= number.start - 1
                            && symbol.index <= number.end + 1
                            && symbol.line >= number.line - 1
                            && symbol.line <= number.line + 1
                    })
                    .is_some()
            })
            .map(|number| number.number)
            .sum();
        format!("{value}")
    }

    fn solve_part2(&self, data: Vec<String>) -> String {
        let numbers = NumberPos::from_data(&data);
        let symbols = SymbolPos::from_data(&data);

        let value: i32 = symbols
            .into_iter()
            .filter_map(|symbol| {
                if symbol.ch == '*' {
                    let near_numbers: Vec<_> = numbers
                        .iter()
                        .filter(|&number| {
                            symbol.index >= number.start - 1
                                && symbol.index <= number.end + 1
                                && symbol.line >= number.line - 1
                                && symbol.line <= number.line + 1
                        })
                        .collect();
                    if near_numbers.len() == 2 {
                        return Some(
                            near_numbers.get(0).unwrap().number
                                * near_numbers.get(1).unwrap().number,
                        );
                    }
                    return None;
                }

                None
            })
            .sum();
        format!("{value}")
    }
}

#[derive(Debug, PartialEq)]
struct NumberPos {
    number: i32,
    start: i32,
    end: i32,
    line: i32,
}

impl NumberPos {
    fn from_data(data: &Vec<String>) -> Vec<Self> {
        data.iter().enumerate().flat_map(Self::from_line).collect()
    }

    fn from_line(data: (usize, &String)) -> Vec<Self> {
        let (line_index, line) = data;
        line.char_indices()
            .fold(
                Vec::new(),
                |mut acc: Vec<(String, usize, usize)>, (idx, ch)| {
                    if ch.is_ascii_digit() {
                        if let Some(last) = acc.last_mut() {
                            let end = last.2;

                            // check if the digits are consecutives
                            if idx == end + 1 {
                                last.0.push(ch);
                                last.2 = idx;
                            } else {
                                acc.push((ch.to_string(), idx, idx));
                            }

                            return acc;
                        }

                        return vec![(ch.to_string(), idx, idx)];
                    }

                    acc
                },
            )
            .into_iter()
            .filter_map(|(s, start, end)| {
                Some(NumberPos {
                    number: s.parse::<i32>().ok()?,
                    start: start as i32,
                    end: end as i32,
                    line: line_index as i32,
                })
            })
            .collect()
    }
}

#[derive(Debug, PartialEq)]
struct SymbolPos {
    ch: char,
    line: i32,
    index: i32,
}

impl SymbolPos {
    fn from_data(data: &Vec<String>) -> Vec<Self> {
        data.iter().enumerate().flat_map(Self::from_line).collect()
    }

    fn from_line(data: (usize, &String)) -> Vec<Self> {
        let (line_index, line) = data;
        line.char_indices()
            .filter_map(|(idx, ch)| {
                if !ch.is_ascii_digit() && ch != '.' {
                    Some(SymbolPos {
                        ch,
                        line: line_index as i32,
                        index: idx as i32,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, "467..114..", vec![
        NumberPos {
            number: 467,
            start: 0,
            end: 2,
            line: 1
        },
        NumberPos {
            number: 114,
            start: 5,
            end: 7,
            line: 1
        }])
    ]
    #[case(1, ".....114..", vec![
        NumberPos {
            number: 114,
            start: 5,
            end: 7,
            line: 1
        }])
    ]
    fn test_number_pos_from_line(
        #[case] line_no: usize,
        #[case] line: String,
        #[case] expected: Vec<NumberPos>,
    ) {
        let numers: Vec<_> = NumberPos::from_line((line_no, &line));

        assert_eq!(numers, expected);
    }

    #[test]
    fn test_number_pos_from_data() {
        let numbers: Vec<_> = NumberPos::from_data(&vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
        ]);

        assert_eq!(
            numbers,
            vec![
                NumberPos {
                    number: 467,
                    start: 0,
                    end: 2,
                    line: 0
                },
                NumberPos {
                    number: 114,
                    start: 5,
                    end: 7,
                    line: 0
                },
                NumberPos {
                    number: 35,
                    start: 2,
                    end: 3,
                    line: 2
                },
                NumberPos {
                    number: 633,
                    start: 6,
                    end: 8,
                    line: 2
                },
            ]
        );
    }

    #[test]
    fn test_symbol_pos_from_line() {
        let symbols: Vec<_> = SymbolPos::from_line((4, &"...$.*....".to_string()));

        assert_eq!(
            symbols,
            vec![
                SymbolPos {
                    ch: '$',
                    index: 3,
                    line: 4
                },
                SymbolPos {
                    ch: '*',
                    index: 5,
                    line: 4
                }
            ]
        );
    }

    #[test]
    fn test_symbol_pos_from_data() {
        let symbol: Vec<_> = SymbolPos::from_data(&vec![
            "617*......".to_string(),
            ".....+.58.".to_string(),
            ".592.....".to_string(),
        ]);

        assert_eq!(
            symbol,
            vec![
                SymbolPos {
                    ch: '*',
                    index: 3,
                    line: 0
                },
                SymbolPos {
                    ch: '+',
                    index: 5,
                    line: 1
                }
            ]
        );
    }
}
