fn parse_calories_per_elf(input: &str) -> Vec<i32> {
    input
        .split("\n\n")
        .map(|x| x.lines().map(|y| y.parse::<i32>().unwrap()).sum::<i32>())
        .collect()
}

pub fn solve_day01_star1(input: &str) -> i32 {
    parse_calories_per_elf(input).into_iter().max().unwrap()
}

pub fn solve_day01_star2(input: &str) -> i32 {
    let mut calories_per_elf = parse_calories_per_elf(input);

    calories_per_elf.sort();
    calories_per_elf.reverse();
    calories_per_elf.iter().take(3).sum()
}

fn parse_rps_hands(input: &str) -> Vec<(&str, &str)> {
    input.lines().map(|x| x.split_once(' ').unwrap()).collect()
}

pub fn solve_day02_star1(input: &str) -> i32 {
    parse_rps_hands(input)
        .iter()
        .map(|x| match x {
            ("A", "X") => 4,
            ("A", "Y") => 8,
            ("A", "Z") => 3,
            ("B", "X") => 1,
            ("B", "Y") => 5,
            ("B", "Z") => 9,
            ("C", "X") => 7,
            ("C", "Y") => 2,
            ("C", "Z") => 6,
            _ => unreachable!(),
        })
        .sum()
}

pub fn solve_day02_star2(input: &str) -> i32 {
    parse_rps_hands(input)
        .iter()
        .map(|x| match x {
            ("A", "X") => 3,
            ("A", "Y") => 4,
            ("A", "Z") => 8,
            ("B", "X") => 1,
            ("B", "Y") => 5,
            ("B", "Z") => 9,
            ("C", "X") => 2,
            ("C", "Y") => 6,
            ("C", "Z") => 7,
            _ => unreachable!(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01_star1() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(solve_day01_star1(input), 24_000);
    }

    #[test]
    fn test_day01_star2() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(solve_day01_star2(input), 45_000);
    }

    #[test]
    fn test_day02_star1() {
        let input = "A Y
B X
C Z";

        assert_eq!(solve_day02_star1(input), 15);
    }

    #[test]
    fn test_day02_star2() {
        let input = "A Y
B X
C Z";

        assert_eq!(solve_day02_star2(input), 12);
    }
}
