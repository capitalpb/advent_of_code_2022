use crate::Solver;

pub struct Day02 {}

impl Solver for Day02 {
    fn star_one(&self, input: &str) -> String {
        input
            .lines()
            .map(|round| match round {
                "A X" => 4,
                "A Y" => 8,
                "A Z" => 3,
                "B X" => 1,
                "B Y" => 5,
                "B Z" => 9,
                "C X" => 7,
                "C Y" => 2,
                "C Z" => 6,
                _ => unreachable!(),
            })
            .sum::<u32>()
            .to_string()
    }

    fn star_two(&self, input: &str) -> String {
        input
            .lines()
            .map(|round| match round {
                "A X" => 3,
                "A Y" => 4,
                "A Z" => 8,
                "B X" => 1,
                "B Y" => 5,
                "B Z" => 9,
                "C X" => 2,
                "C Y" => 6,
                "C Z" => 7,
                _ => unreachable!(),
            })
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "A Y
B X
C Z";

    #[test]
    fn test_star_one() {
        let solver = Day02 {};
        assert_eq!(solver.star_one(TEST_DATA), "15");
    }

    #[test]
    fn test_star_two() {
        let solver = Day02 {};
        assert_eq!(solver.star_two(TEST_DATA), "12");
    }
}
