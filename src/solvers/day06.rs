use std::collections::BTreeSet;

use crate::Solver;

pub struct Day06;

fn find_marker_index(input: &str, unique_characters: usize) -> String {
    for (i, window) in input
        .chars()
        .collect::<Vec<_>>()
        .windows(unique_characters)
        .enumerate()
    {
        let window_set = BTreeSet::from_iter(window.iter());
        if window_set.len() == unique_characters {
            return (i + unique_characters).to_string();
        }
    }
    unreachable!()
}

impl Solver for Day06 {
    fn star_one(&self, input: &str) -> String {
        find_marker_index(input, 4)
    }

    fn star_two(&self, input: &str) -> String {
        find_marker_index(input, 14)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn test_star_one() {
        let expected = ["7", "5", "6", "10", "11"];
        let solver = Day06 {};

        for i in 0..5 {
            assert_eq!(solver.star_one(TEST_DATA[i]), expected[i]);
        }
    }

    #[test]
    fn test_star_two() {
        let expected = ["19", "23", "23", "29", "26"];
        let solver = Day06 {};

        for i in 0..5 {
            assert_eq!(solver.star_two(TEST_DATA[i]), expected[i]);
        }
    }
}
