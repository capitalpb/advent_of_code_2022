use crate::Solver;

pub struct Day04;

fn parse_sections_to_u32s(section_pair: &str) -> ((u32, u32), (u32, u32)) {
    let (section1, section2) = section_pair.split_once(',').unwrap();

    let section1 = section1.split_once('-').unwrap();
    let section2 = section2.split_once('-').unwrap();

    let section1 = (
        section1.0.parse::<u32>().unwrap(),
        section1.1.parse::<u32>().unwrap(),
    );

    let section2 = (
        section2.0.parse::<u32>().unwrap(),
        section2.1.parse::<u32>().unwrap(),
    );

    (section1, section2)
}

impl Solver for Day04 {
    fn star_one(&self, input: &str) -> String {
        input
            .lines()
            .map(|pair| parse_sections_to_u32s(pair))
            .filter(|pair| {
                (pair.0 .0 >= pair.1 .0 && pair.0 .1 <= pair.1 .1)
                    || (pair.1 .0 >= pair.0 .0 && pair.1 .1 <= pair.0 .1)
            })
            .count()
            .to_string()
    }

    fn star_two(&self, input: &str) -> String {
        input
            .lines()
            .map(|pair| parse_sections_to_u32s(pair))
            .filter(|pair| {
                ((pair.0 .0 >= pair.1 .0 || pair.0 .1 >= pair.1 .0) && pair.0 .1 <= pair.1 .1)
                    || ((pair.1 .0 >= pair.0 .0 || pair.1 .1 >= pair.0 .0)
                        && pair.1 .1 <= pair.0 .1)
            })
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_star_one() {
        let solver = Day04 {};
        assert_eq!(solver.star_one(TEST_DATA), "2");
    }

    #[test]
    fn test_star_two() {
        let solver = Day04 {};
        assert_eq!(solver.star_two(TEST_DATA), "4");
    }
}
