mod y2023;

use std::{collections::HashMap, fmt::Debug};

pub trait Solver: Debug {
    fn solve_part1(&self, data: Vec<String>) -> String;
    fn solve_part2(&self, data: Vec<String>) -> String;
}

pub struct SolutionsMap<'a> {
    solutions: HashMap<&'a str, Box<dyn Solver>>,
}

impl<'a> SolutionsMap<'a> {
    pub fn init() -> Self {
        let mut map: HashMap<&'a str, Box<dyn Solver>> = HashMap::new();
        // register the solutions here
        map.insert("2023_1", y2023::day_1::Solution::boxed());
        map.insert("2023_2", y2023::day_2::Solution::boxed());
        //
        Self { solutions: map }
    }

    pub fn get(self: &Self, entry: &str) -> Option<&Box<dyn Solver>> {
        self.solutions.get(entry)
    }
}
