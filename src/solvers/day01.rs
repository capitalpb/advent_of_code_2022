use crate::Solver;

pub struct Day01 {}

impl Day01 {
    fn calories_per_elf(&self, input: &str) -> Vec<u32> {
        input
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|cals| cals.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .collect()
    }
}

impl Solver for Day01 {
    fn star_one(&self, input: &str) -> String {
        self.calories_per_elf(input)
            .iter()
            .max()
            .unwrap()
            .to_string()
    }

    fn star_two(&self, input: &str) -> String {
        let mut calories_per_elf = self.calories_per_elf(input);
        calories_per_elf.sort();

        calories_per_elf
            .iter()
            .rev()
            .take(3)
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_star_one() {
        let solver = Day01 {};
        assert_eq!(solver.star_one(TEST_DATA), "24000");
    }

    #[test]
    fn test_star_two() {
        let solver = Day01 {};
        assert_eq!(solver.star_two(TEST_DATA), "45000");
    }
}
