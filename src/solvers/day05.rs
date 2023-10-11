use crate::Solver;

pub struct Day05;

fn parse_input(input: &str) -> (Vec<Vec<char>>, &str) {
    let (starting_stacks, instructions) = input.split_once("\n\n").unwrap();

    let mut starting_stacks_iterator = starting_stacks.lines().rev();

    let stack_count = starting_stacks_iterator
        .next()
        .unwrap()
        .trim()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;

    let mut stacks: Vec<Vec<char>> = vec![vec![]; stack_count];
    for stack in starting_stacks_iterator {
        let stack_chars: Vec<_> = stack.chars().collect();
        for i in 0..stack_count {
            let c = stack_chars[i * 4 + 1];
            if !c.is_whitespace() {
                stacks[i].push(c);
            }
        }
    }

    (stacks, instructions)
}

fn parse_instruction(instruction: &str) -> Vec<usize> {
    instruction
        .split(" ")
        .filter(|s| s.chars().all(|c| c.is_digit(10)))
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn get_stack_tops(stacks: &Vec<Vec<char>>) -> String {
    stacks.iter().filter_map(|stack| stack.last()).collect()
}

impl Solver for Day05 {
    fn star_one(&self, input: &str) -> String {
        let (mut stacks, instructions) = parse_input(input);

        for instruction in instructions.lines() {
            let numbers = parse_instruction(instruction);

            for _ in 0..numbers[0] {
                let x = stacks[numbers[1] - 1].pop().unwrap();
                stacks[numbers[2] - 1].push(x);
            }
        }

        get_stack_tops(&stacks)
    }

    fn star_two(&self, input: &str) -> String {
        let (mut stacks, instructions) = parse_input(input);

        for instruction in instructions.lines() {
            let numbers = parse_instruction(instruction);

            let new_length = stacks[numbers[1] - 1].len() - numbers[0];
            let mut crates = stacks[numbers[1] - 1].split_off(new_length);
            stacks[numbers[2] - 1].append(&mut crates);
        }

        get_stack_tops(&stacks)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_star_one() {
        let solver = Day05 {};
        assert_eq!(solver.star_one(TEST_DATA), "CMZ");
    }

    #[test]
    fn test_star_two() {
        let solver = Day05 {};
        assert_eq!(solver.star_two(TEST_DATA), "MCD")
    }
}
