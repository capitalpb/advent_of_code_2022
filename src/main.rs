use std::fs;

use advent_of_code_2022::*;

fn main() {
    println!("Advent of Code 2022");
    println!("===================");

    let input = fs::read_to_string("inputs/day01.txt").unwrap();
    let star1 = solve_day01_star1(&input);
    let star2 = solve_day01_star2(&input);
    println!("Day 01 Star 1: {star1}");
    println!("Day 01 Star 2: {star2}");

    let input = fs::read_to_string("inputs/day02.txt").unwrap();
    let star1 = solve_day02_star1(&input);
    let star2 = solve_day02_star2(&input);
    println!("Day 02 Star 1: {star1}");
    println!("Day 02 Star 2: {star2}");
}
