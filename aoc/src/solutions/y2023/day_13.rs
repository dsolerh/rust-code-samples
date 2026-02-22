use std::mem;

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
        todo!()
    }

    fn solve_part2(&self, data: Vec<String>) -> String {
        todo!()
    }
}

fn is_symetric(chain: &Vec<char>, start: usize) -> bool {
    let mut current = start;
    let mut next = current + 1;

    loop {
        match (chain.get(current), chain.get(next)) {
            (Some(&x1), Some(&x2)) if x1 == x2 => {
                if current == 0 {
                    return true;
                }
                current -= 1;
                next += 1;
            }
            (Some(_), Some(_)) => {
                return false;
            }
            (_, _) => {
                return true;
            }
        }
    }
}

fn find_symmetry(grid: &Vec<Vec<char>>) -> Option<usize> {
    let cols = grid[0].len();
    for col in 0..cols - 1 {
        if grid.iter().all(|row| is_symetric(row, col)) {
            return Some(col + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("#..#", 0, false)]
    #[case("#..#", 1, true)]
    #[case(".#..##", 2, false)]
    fn test_is_symmetric(#[case] line: &str, #[case] start: usize, #[case] expect_symetric: bool) {
        let chain = line.chars().collect::<Vec<_>>();

        assert_eq!(is_symetric(&chain, start), expect_symetric);
    }

    #[rstest]
    #[case(
        vec![
            "#.##..##.".chars().collect::<Vec<_>>(),
            "..#.##.#.".chars().collect::<Vec<_>>(),
            "##......#".chars().collect::<Vec<_>>(),
            "##......#".chars().collect::<Vec<_>>(),
            "..#.##.#.".chars().collect::<Vec<_>>(),
            "..##..##.".chars().collect::<Vec<_>>(),
        ],
        Some(5)
    )]
    #[case(
        vec![
            ".##..##.".chars().collect::<Vec<_>>(),
            ".#.##.#.".chars().collect::<Vec<_>>(),
            "#......#".chars().collect::<Vec<_>>(),
            "#......#".chars().collect::<Vec<_>>(),
            ".#.##.#.".chars().collect::<Vec<_>>(),
            ".##..##.".chars().collect::<Vec<_>>(),
        ],
        Some(4)
    )]
    #[case(
        vec![
            ".##..##.".chars().collect::<Vec<_>>(),
            ".#..#.#.".chars().collect::<Vec<_>>(),
            "#..#...#".chars().collect::<Vec<_>>(),
        ],
        None
    )]
    fn test_find_vertical_symmetry(#[case] grid: Vec<Vec<char>>, #[case] expected: Option<usize>) {
        let got = find_symmetry(&grid);
        assert_eq!(got, expected);
    }
}
