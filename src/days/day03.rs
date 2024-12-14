use crate::{Solution, SolutionPair};
use regex::Regex;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day03.txt").unwrap();
    fn solve_part1(input: &str) -> u64 {
        let reggie = Regex::new(r"^\((\d+),(\d+)\)").unwrap();
        let split_input: Vec<&str> = input.split("mul").collect();
        // println!("{:?}", split_input);
        // println!("{}", reggie);
        let mut result: u64 = 0;
        for i in 0..split_input.len() {
            let mut product: Option<u64> = None;
            if let Some(captures) = reggie.captures(split_input[i]) {
                let first_number: u64 = captures.get(1).unwrap().as_str().parse().unwrap();
                let second_number: u64 = captures.get(2).unwrap().as_str().parse().unwrap();
                product = Some(first_number * second_number);
                // println!("Found: ({}, {})", first_number, second_number);
            } else {
                // println!("nothing found");
            }
            if let Some(prod) = product {
                result += prod;
            }
        }
        result
    }

    fn solve_part2(input: &str) -> u64 {
        let mul_reg = Regex::new(r"^\((\d+),(\d+)\)").unwrap();
        let mut result: u64 = 0;

        let split_do: Vec<&str> = input
            .split("do()")
            .map(|s| s.split("don't()").next().unwrap_or(""))
            .collect();
        let split_do: String = split_do.join("");

        let split_mul: Vec<&str> = split_do.split("mul").collect();

        println!("{:?}", split_mul);

        for i in 0..split_mul.len() {
            let mut product: Option<u64> = None;
            if let Some(captures) = mul_reg.captures(split_mul[i]) {
                let first_number: u64 = captures.get(1).unwrap().as_str().parse().unwrap();
                let second_number: u64 = captures.get(2).unwrap().as_str().parse().unwrap();
                product = Some(first_number * second_number);
                // println!("Found: ({}, {})", first_number, second_number);
            } else {
                // println!("nothing found");
            }
            if let Some(prod) = product {
                result += prod;
            }
        }

        result
    }

    // Your solution here...
    let sol1: u64 = solve_part1(&input);
    let sol2: u64 = solve_part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}
