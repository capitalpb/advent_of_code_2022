use crate::Solver;

pub struct Day10;

fn plot_x_history(input: &str) -> Vec<i32> {
    let mut x_history = vec![1];

    for instruction in input.lines() {
        let x = x_history.last().unwrap().clone();
        x_history.push(x);

        if instruction != "noop" {
            let value = instruction
                .split_once(' ')
                .unwrap()
                .1
                .parse::<i32>()
                .unwrap();

            x_history.push(x + value);
        }
    }

    x_history
}

impl Solver for Day10 {
    fn star_one(&self, input: &str) -> String {
        let x_history = plot_x_history(input);

        [20, 60, 100, 140, 180, 220]
            .map(|cycle| (cycle as i32) * x_history[cycle - 1])
            .iter()
            .sum::<i32>()
            .to_string()
    }

    fn star_two(&self, input: &str) -> String {
        let x_history = plot_x_history(input);
        let mut cycle = 0;
        let mut crt_display = String::from("");

        for _ in 0..6 {
            crt_display.push_str("\n");

            for col in 0..40 {
                crt_display.push_str(if (x_history[cycle] - col).abs() < 2 {
                    "#"
                } else {
                    "."
                });

                cycle += 1;
            }
        }

        crt_display
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_star_one() {
        let solver = Day10 {};
        assert_eq!(solver.star_one(TEST_DATA), "13140");
    }

    #[test]
    fn test_star_two() {
        let solver = Day10 {};
        assert_eq!(
            solver.star_two(TEST_DATA),
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
