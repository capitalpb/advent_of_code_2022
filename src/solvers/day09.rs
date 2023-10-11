use std::collections::HashSet;

use crate::Solver;

pub struct Day09;

impl Solver for Day09 {
    fn star_one(&self, input: &str) -> String {
        let mut head_pos = (0, 0);
        let mut tail_pos = (0, 0);
        let mut visited_by_tail = HashSet::from([tail_pos]);

        for instruction in input.lines() {
            let (direction, steps) = instruction.split_once(' ').unwrap();
            let steps = steps.parse::<i32>().unwrap();

            for _ in 0..steps {
                match direction {
                    "R" => head_pos.0 += 1,
                    "L" => head_pos.0 -= 1,
                    "U" => head_pos.1 -= 1,
                    "D" => head_pos.1 += 1,
                    _ => unreachable!(),
                };

                let x_diff = head_pos.0 - tail_pos.0;
                let y_diff = head_pos.1 - tail_pos.1;

                if x_diff == 2 && y_diff == 0 {
                    tail_pos.0 += 1;
                } else if x_diff == -2 && y_diff == 0 {
                    tail_pos.0 -= 1;
                } else if y_diff == 2 && x_diff == 0 {
                    tail_pos.1 += 1;
                } else if y_diff == -2 && x_diff == 0 {
                    tail_pos.1 -= 1;
                } else if x_diff == 2 && y_diff == 1 {
                    tail_pos.0 += 1;
                    tail_pos.1 += 1;
                } else if x_diff == 2 && y_diff == -1 {
                    tail_pos.0 += 1;
                    tail_pos.1 -= 1;
                } else if x_diff == -2 && y_diff == 1 {
                    tail_pos.0 -= 1;
                    tail_pos.1 += 1;
                } else if x_diff == -2 && y_diff == -1 {
                    tail_pos.0 -= 1;
                    tail_pos.1 -= 1;
                } else if y_diff == 2 && x_diff == 1 {
                    tail_pos.0 += 1;
                    tail_pos.1 += 1;
                } else if y_diff == 2 && x_diff == -1 {
                    tail_pos.0 -= 1;
                    tail_pos.1 += 1;
                } else if y_diff == -2 && x_diff == 1 {
                    tail_pos.0 += 1;
                    tail_pos.1 -= 1;
                } else if y_diff == -2 && x_diff == -1 {
                    tail_pos.0 -= 1;
                    tail_pos.1 -= 1;
                }

                visited_by_tail.insert(tail_pos);
            }
        }

        visited_by_tail.len().to_string()
    }

    fn star_two(&self, input: &str) -> String {
        todo!()
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
}
