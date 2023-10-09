use std::collections::HashMap;

use crate::Solver;

pub struct Day07 {}

type Directories = HashMap<String, HashMap<String, usize>>;

fn parse_file_system(input: &str) -> Directories {
    let mut directories = HashMap::new();
    let mut current_dir = String::from("");

    for line in input.lines() {
        match line {
            "$ cd /" => {
                current_dir = String::from("/");
                directories
                    .entry(current_dir.clone())
                    .or_insert(HashMap::new());
            }

            "$ cd .." => {
                let parent_length = current_dir.rfind('/').unwrap();
                current_dir.truncate(parent_length);
            }

            line if line.starts_with("$ cd") => {
                let split_line: Vec<_> = line.split(' ').collect();
                current_dir.push_str(&format!("/{}", split_line[2]));
            }

            line if line.starts_with("dir") => {
                let split_line: Vec<_> = line.split(' ').collect();
                directories
                    .entry(format!("{current_dir}/{}", split_line[1]))
                    .or_insert(HashMap::new());
            }

            _ => {
                let split_line: Vec<_> = line.split(' ').collect();
                if let Ok(size) = split_line[0].parse::<usize>() {
                    directories.entry(current_dir.clone()).and_modify(|files| {
                        files.insert(String::from(split_line[1]), size);
                    });
                }
            }
        }
    }

    directories
}

fn calculate_directory_sizes(directories: &Directories) -> HashMap<String, usize> {
    directories
        .keys()
        .map(|dir| {
            let mut dir_size = 0;
            for (directory, files) in directories {
                if directory.starts_with(dir) {
                    dir_size += files.values().sum::<usize>();
                }
            }
            (dir.clone(), dir_size)
        })
        .collect()
}

impl Solver for Day07 {
    fn star_one(&self, input: &str) -> String {
        let directories = parse_file_system(input);

        calculate_directory_sizes(&directories)
            .into_values()
            .filter(|size| size <= &100_000)
            .sum::<usize>()
            .to_string()
    }

    fn star_two(&self, input: &str) -> String {
        let directories = parse_file_system(input);
        let directory_sizes = calculate_directory_sizes(&directories);

        let required_space: usize = 30_000_000 - (70_000_000 - directory_sizes.get("/").unwrap());

        let mut suitable_sizes: Vec<_> = directory_sizes
            .into_values()
            .filter(|size| size >= &required_space)
            .collect();
        suitable_sizes.sort();

        suitable_sizes.first().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_star_one() {
        let solver = Day07 {};
        assert_eq!(solver.star_one(TEST_DATA), "95437");
    }

    #[test]
    fn test_star_two() {
        let solver = Day07 {};
        assert_eq!(solver.star_two(TEST_DATA), "24933642");
    }
}
