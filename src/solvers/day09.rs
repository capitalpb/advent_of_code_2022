use std::collections::HashSet;

use crate::Solver;

pub struct Day09;

fn solve(input: &str, knot_count: usize) -> String {
    let mut knots = vec![(0, 0); knot_count];
    let mut visited_by_tail = HashSet::from([*knots.last().unwrap()]);

    for instruction in input.lines() {
        let (direction, steps) = instruction.split_once(' ').unwrap();
        let steps = steps.parse::<i32>().unwrap();

        for _ in 0..steps {
            match direction {
                "R" => knots[0].0 += 1,
                "L" => knots[0].0 -= 1,
                "U" => knots[0].1 -= 1,
                "D" => knots[0].1 += 1,
                _ => unreachable!(),
            };

            for i in 1..knots.len() {
                let x_diff: i32 = knots[i - 1].0 - knots[i].0;
                let y_diff: i32 = knots[i - 1].1 - knots[i].1;

                if x_diff.abs() == 2 {
                    knots[i].0 += x_diff / 2;

                    if y_diff.abs() == 1 {
                        knots[i].1 += y_diff;
                    }
                }

                if y_diff.abs() == 2 {
                    knots[i].1 += y_diff / 2;

                    if x_diff.abs() == 1 {
                        knots[i].0 += x_diff;
                    }
                }
            }

            visited_by_tail.insert(*knots.last().unwrap());
        }
    }

    visited_by_tail.len().to_string()
}

impl Solver for Day09 {
    fn star_one(&self, input: &str) -> String {
        solve(input, 2)
    }

    fn star_two(&self, input: &str) -> String {
        solve(input, 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_star_one() {
        let solver = Day09 {};
        assert_eq!(solver.star_one(TEST_DATA), "13");
    }

    #[test]
    fn test_star_two() {
        let solver = Day09 {};
        assert_eq!(solver.star_two(TEST_DATA), "1");

        assert_eq!(
            solver.star_two(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            ),
            "36"
        );
    }
}
