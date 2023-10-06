use std::fs;

use advent_of_code_2022::init_solver;

fn main() {
    println!("Advent of Code 2022");
    println!("===================");

    for day in 1.. {
        if let Some(solver) = init_solver(day) {
            let input = fs::read_to_string(format!("inputs/day{day:0>2}.txt")).unwrap();
            println!("");
            println!("Day {day:0>2} Star 1: {}", solver.star_one(&input));
            println!("Day {day:0>2} Star 2: {}", solver.star_two(&input));
        } else {
            break;
        }
    }
}
