mod solvers;

use crate::solvers::{
    day01::Day01, day02::Day02, day03::Day03, day04::Day04, day05::Day05, day06::Day06,
    day07::Day07,
};

pub trait Solver {
    fn star_one(&self, input: &str) -> String;
    fn star_two(&self, input: &str) -> String;
}

pub fn init_solver(day: u8) -> Option<Box<dyn Solver>> {
    match day {
        1 => Some(Box::new(Day01 {})),
        2 => Some(Box::new(Day02 {})),
        3 => Some(Box::new(Day03 {})),
        4 => Some(Box::new(Day04 {})),
        5 => Some(Box::new(Day05 {})),
        6 => Some(Box::new(Day06 {})),
        7 => Some(Box::new(Day07 {})),
        _ => None,
    }
}
