use crate::Solver;

pub struct Day11;

struct Monkey {
    items: Vec<i64>,
    operation: char,
    operand1: Option<i64>,
    operand2: Option<i64>,
    test_divisor: i64,
    throw_if_true: usize,
    throw_if_false: usize,
    inspections: usize,
}

impl Monkey {
    fn from(monkey_string: &str) -> Monkey {
        let mut lines = monkey_string.lines().skip(1);

        let items: Vec<_> = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|item| item.parse::<i64>().unwrap())
            .collect();

        let operation_parts: Vec<_> = lines
            .next()
            .unwrap()
            .split_once("= ")
            .unwrap()
            .1
            .split(" ")
            .collect();

        let operation = operation_parts[1].chars().nth(0).unwrap();
        let operand1 = operation_parts[0].parse::<i64>().ok();
        let operand2 = operation_parts[2].parse::<i64>().ok();

        let test_divisor = lines
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let throw_if_true = lines
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let throw_if_false = lines
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Monkey {
            items,
            operation,
            operand1,
            operand2,
            test_divisor,
            throw_if_true,
            throw_if_false,
            inspections: 0,
        }
    }

    fn inspect(&mut self, value: i64) -> i64 {
        self.inspections += 1;

        let operand1 = self.operand1.unwrap_or(value);
        let operand2 = self.operand2.unwrap_or(value);

        match self.operation {
            '+' => operand1 + operand2,
            '-' => operand1 - operand2,
            '*' => operand1 * operand2,
            '/' => operand1 / operand2,
            _ => unreachable!(),
        }
    }
}

fn solve<C: Fn(i64) -> i64>(mut monkies: Vec<Monkey>, rounds: usize, worry_modifier: C) -> String {
    for _ in 0..rounds {
        for i in 0..monkies.len() {
            for mut item in monkies[i].items.drain(..).collect::<Vec<_>>() {
                item = monkies[i].inspect(item);
                item = worry_modifier(item);

                let test_passed = item % monkies[i].test_divisor == 0;
                let throw_to = if test_passed {
                    monkies[i].throw_if_true
                } else {
                    monkies[i].throw_if_false
                };

                monkies[throw_to].items.push(item);
            }
        }
    }

    let mut inspections: Vec<_> = monkies.iter().map(|monkey| monkey.inspections).collect();
    inspections.sort();

    inspections
        .into_iter()
        .rev()
        .take(2)
        .reduce(|a, b| a * b)
        .unwrap()
        .to_string()
}

impl Solver for Day11 {
    fn star_one(&self, input: &str) -> String {
        let monkies: Vec<_> = input.split("\n\n").map(Monkey::from).collect();

        solve(monkies, 20, |item| item / 3)
    }

    fn star_two(&self, input: &str) -> String {
        let monkies: Vec<_> = input.split("\n\n").map(Monkey::from).collect();

        let worry_mod = monkies
            .iter()
            .map(|monkey| monkey.test_divisor)
            .collect::<Vec<_>>()
            .into_iter()
            .reduce(|a, b| a * b)
            .unwrap();

        solve(monkies, 10000, |item| item % worry_mod)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_star_one() {
        let solver = Day11 {};
        assert_eq!(solver.star_one(TEST_DATA), "10605");
    }

    #[test]
    fn test_star_two() {
        let solver = Day11 {};
        assert_eq!(solver.star_two(TEST_DATA), "2713310158");
    }
}
