use crate::{Solution, SolutionPair};
use std::{collections::HashMap, fs::read_to_string};

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

    fn part2(input: &str) -> u64 {
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

        let mut right_count: HashMap<u64, u64> = HashMap::new();

        for i in 0..right_side.len() {
            *right_count.entry(right_side[i]).or_insert(0) += 1;
        }

        let mut result = 0;
        for key in left_side {
            if let Some(right_value) = right_count.get(&key) {
                // println!("{}: {} in right_count", key, right_value);
                result += key * right_value;
            } else {
                // println!("{}: 0 in right_count", key);
            }
        }

        result
    }

    // Your solution here...
    let sol1: u64 = part1(&input);
    let sol2: u64 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}
