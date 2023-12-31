mod solvers;

use crate::solvers::*;

pub trait Solver {
    fn star_one(&self, input: &str) -> String;
    fn star_two(&self, input: &str) -> String;
}

pub fn init_solver(day: u8) -> Option<Box<dyn Solver>> {
    match day {
        1 => Some(Box::new(day01::Day01 {})),
        2 => Some(Box::new(day02::Day02 {})),
        3 => Some(Box::new(day03::Day03 {})),
        4 => Some(Box::new(day04::Day04 {})),
        5 => Some(Box::new(day05::Day05 {})),
        6 => Some(Box::new(day06::Day06 {})),
        7 => Some(Box::new(day07::Day07 {})),
        8 => Some(Box::new(day08::Day08 {})),
        9 => Some(Box::new(day09::Day09 {})),
        10 => Some(Box::new(day10::Day10 {})),
        11 => Some(Box::new(day11::Day11 {})),
        _ => None,
    }
}
