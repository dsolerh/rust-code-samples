use crate::solutions::Solver;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Solver for Solution {
    fn solve_part1(&self, _data: Vec<String>) -> String {
        format!("{}", 1)
    }

    fn solve_part2(&self, _data: Vec<String>) -> String {
        format!("{}", 1)
    }
}
