use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day02.txt").unwrap();
    let input_lines = input.lines().collect::<Vec<&str>>();
    let input_lines_collected: Vec<Vec<i32>> = input_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    pub fn solve_part1(input: &Vec<Vec<i32>>) -> u64 {
        let mut total: u64 = 0;

        for line in input {
            let mut increasing = true;
            let mut decreasing = true;
            const MAX_INCREMENT: i32 = 3;
            let mut small_increments = true;

            for i in 0..line.len() - 1 {
                let diff = line[i] - line[i + 1];
                if diff.abs() > MAX_INCREMENT {
                    small_increments = false;
                }
                if line[i] < line[i + 1] {
                    decreasing = false;
                } else if line[i] > line[i + 1] {
                    increasing = false;
                } else {
                    decreasing = false;
                    increasing = false;
                }
            }
            // if !increasing && !decreasing {
            //     println!("not increasing or decreasing");
            // } else if increasing {
            //     println!("increasing");
            // } else if decreasing {
            //     println!("decreasing");
            // }
            if (increasing || decreasing) && small_increments {
                total += 1;
                // println!("{:?} = Safe", line);
            } else {
                // println!("{:?} = Not safe", line);
            }
        }

        total
    }

    fn is_safe(line: &[i32]) -> bool {
        if line.len() < 2 {
            return true;
        }

        const MAX_INCREMENT: i32 = 3;

        let mut is_increasing = true;
        let mut is_decreasing = true;

        for i in 0..line.len() - 1 {
            let diff = line[i + 1] - line[i];

            if diff > 0 && diff <= MAX_INCREMENT {
                is_decreasing = false;
            } else if diff < 0 && diff >= -MAX_INCREMENT {
                is_increasing = false;
            } else {
                return false;
            }
        }

        is_increasing || is_decreasing
    }

    pub fn solve_part2(input: &[Vec<i32>]) -> u64 {
        let mut total: u64 = 0;

        for line in input {
            if is_safe(line) {
                total += 1;
                // println!("{:?} = Safe", line);
            } else {
                let mut found_safe = false;
                for i in 0..line.len() {
                    let mut check_line = line.clone();
                    check_line.remove(i);

                    if is_safe(&check_line) {
                        total += 1;
                        found_safe = true;
                        // println!("{:?} = Safe", check_line);
                        break;
                    }
                }
                if !found_safe {
                    // println!("{:?} = Unsafe", line);
                }
            }
        }

        total
    }

    // Your solution here...
    let sol1: u64 = solve_part1(&input_lines_collected);
    let sol2: u64 = solve_part2(&input_lines_collected);

    (Solution::from(sol1), Solution::from(sol2))
}
