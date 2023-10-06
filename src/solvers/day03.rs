use crate::Solver;

pub struct Day03 {}

fn item_priority(item: char) -> u32 {
    (item as u32) - (if item.is_uppercase() { 38 } else { 96 })
}

impl Solver for Day03 {
    fn star_one(&self, input: &str) -> String {
        input
            .lines()
            .map(|sack| {
                let (compartment1, compartment2) = sack.split_at(sack.len() / 2);
                for item in compartment1.chars() {
                    if compartment2.contains(item) {
                        return item_priority(item);
                    }
                }
                unreachable!()
            })
            .sum::<u32>()
            .to_string()
    }

    fn star_two(&self, input: &str) -> String {
        input
            .lines()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|group| {
                for item in group[0].chars() {
                    if group[1].contains(item) && group[2].contains(item) {
                        return item_priority(item);
                    }
                }
                unreachable!()
            })
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_star_one() {
        let solver = Day03 {};
        assert_eq!(solver.star_one(TEST_DATA), "157");
    }

    #[test]
    fn test_star_two() {
        let solver = Day03 {};
        assert_eq!(solver.star_two(TEST_DATA), "70");
    }
}
