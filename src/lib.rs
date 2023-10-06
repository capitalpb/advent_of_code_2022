mod solvers;

use crate::solvers::{day01::Day01, day02::Day02};

pub trait Solver {
    fn star_one(&self, input: &str) -> String;
    fn star_two(&self, input: &str) -> String;
}

pub fn init_solver(day: u8) -> Option<Box<dyn Solver>> {
    match day {
        1 => Some(Box::new(Day01 {})),
        2 => Some(Box::new(Day02 {})),
        _ => None,
    }
}