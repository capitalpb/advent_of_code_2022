use std::fs;

use advent_of_code_2022::init_solver;

fn main() {
    println!("Advent of Code 2022");
    println!("===================");

    let day = 2;
    let input = fs::read_to_string(format!("inputs/day{day}.txt")).unwrap();

    if let Some(solver) = init_solver(day) {
        println!("Day {day} Star 1: {}", solver.star_one(&input));
        println!("Day {day} Star 2: {}", solver.star_two(&input));
    }
}
