use crate::Solver;

pub struct Day08;

impl Solver for Day08 {
    fn star_one(&self, input: &str) -> String {
        let trees: Vec<_> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();

        let rows = trees.len();
        let cols = trees[0].len();
        let mut visible_trees: usize = 0;

        for row in 0..rows {
            for col in 0..cols {
                if row == 0 || row == (rows - 1) || col == 0 || col == (cols - 1) {
                    visible_trees += 1;
                    continue;
                }

                if trees[row][..col].iter().all(|tree| tree < &trees[row][col]) {
                    visible_trees += 1;
                    continue;
                }

                if trees[row][(col + 1)..]
                    .iter()
                    .all(|tree| tree < &trees[row][col])
                {
                    visible_trees += 1;
                    continue;
                }

                let current_col: Vec<_> = trees.iter().map(|row| row[col]).collect();

                if current_col[..row]
                    .iter()
                    .all(|tree| tree < &trees[row][col])
                {
                    visible_trees += 1;
                    continue;
                }

                if current_col[(row + 1)..]
                    .iter()
                    .all(|tree| tree < &trees[row][col])
                {
                    visible_trees += 1;
                    continue;
                }
            }
        }

        visible_trees.to_string()
    }

    fn star_two(&self, input: &str) -> String {
        let trees: Vec<_> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();

        let rows = trees.len();
        let cols = trees[0].len();
        let mut scenic_scores = Vec::<u32>::new();

        for row in 0..rows {
            for col in 0..cols {
                let mut west_viewing_distance = 0;
                let mut east_viewing_distance = 0;
                let mut north_viewing_distance = 0;
                let mut south_viewing_distance = 0;

                let current_col: Vec<_> = trees.iter().map(|row| row[col]).collect();

                for tree in trees[row][..col].iter().rev() {
                    west_viewing_distance += 1;

                    if tree >= &trees[row][col] {
                        break;
                    }
                }

                for tree in trees[row][(col + 1)..].iter() {
                    east_viewing_distance += 1;

                    if tree >= &trees[row][col] {
                        break;
                    }
                }

                for tree in current_col[..row].iter().rev() {
                    north_viewing_distance += 1;

                    if tree >= &trees[row][col] {
                        break;
                    }
                }

                for tree in current_col[(row + 1)..].iter() {
                    south_viewing_distance += 1;

                    if tree >= &trees[row][col] {
                        break;
                    }
                }

                scenic_scores.push(
                    west_viewing_distance
                        * east_viewing_distance
                        * north_viewing_distance
                        * south_viewing_distance,
                );
            }
        }

        scenic_scores.iter().max().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_star_one() {
        let solver = Day08 {};
        assert_eq!(solver.star_one(TEST_DATA), "21");
    }

    #[test]
    fn test_star_two() {
        let solver = Day08 {};
        assert_eq!(solver.star_two(TEST_DATA), "8");
    }
}
