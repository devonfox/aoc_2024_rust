use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01.txt").unwrap();

    fn part1(input: &str) -> u64 {
        let split_input: Vec<&str> = input.split("\n").collect();
        let mut left_side: Vec<u64> = Vec::new();
        let mut right_side: Vec<u64> = Vec::new();

        for line in split_input {
            let split_ints: Vec<&str> = line.split_whitespace().collect();
            // println!("{:?}", split_ints);
            left_side.push(split_ints[0].parse().unwrap());
            right_side.push(split_ints[1].parse().unwrap());
        }

        left_side.sort();
        right_side.sort();

        let mut result: u64 = 0;

        for i in 0..left_side.len() {
            let mut difference: u64 = 0;
            if left_side[i] > right_side[i] {
                difference += left_side[i] - right_side[i];
            } else {
                difference += right_side[i] - left_side[i];
            }
            result += difference;
        }

        result
    }

    // Your solution here...
    let sol1: u64 = part1(&input);
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
