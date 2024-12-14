use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),  // Up
    (1, 0),   // Down
    (0, -1),  // Left
    (0, 1),   // Right
    (-1, -1), // Up Left
    (-1, 1),  // Up Right
    (1, -1),  // Down Left
    (1, 1),   // DownRight
];

const DIAGONAL_DIRECTIONS: [(isize, isize); 4] = [
    (-1, -1), // Up Left
    (-1, 1),  // Up Right
    (1, -1),  // Down Left
    (1, 1),   // DownRight
];

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day04.txt").unwrap();

    fn check_for_xmas(
        position: (isize, isize),
        parsed_input: &Vec<&str>,
        index: usize,
        direction: (isize, isize),
    ) -> bool {
        let xmas_key: &str = "XMAS";
        let (i, j) = position;

        if i < 0
            || j < 0
            || i >= parsed_input.len() as isize
            || j >= parsed_input[i as usize].len() as isize
        {
            return false;
        }

        if parsed_input[i as usize].chars().nth(j as usize).unwrap()
            != xmas_key.chars().nth(index).unwrap()
        {
            return false;
        }

        if index == 3 {
            return true;
        }

        check_for_xmas(
            (i + direction.0, j + direction.1),
            parsed_input,
            index + 1,
            direction,
        )
    }

    fn solve_part1(input: &str) -> u64 {
        let parsed_input = input.split_whitespace().collect::<Vec<&str>>();
        let mut xmas_count = 0;

        for i in 0..parsed_input.len() {
            for (j, char) in parsed_input[i].chars().enumerate() {
                if char == 'X' {
                    for direction in DIRECTIONS {
                        if check_for_xmas((i as isize, j as isize), &parsed_input, 0, direction) {
                            xmas_count += 1;
                        }
                    }
                }
            }
        }

        xmas_count
    }

    fn check_for_xmas_part2(position: (isize, isize), parsed_input: &Vec<&str>) -> bool {
        let (i, j) = position;
        let (lui, luj) = (i + DIAGONAL_DIRECTIONS[0].0, j + DIAGONAL_DIRECTIONS[0].1);
        let (rui, ruj) = (i + DIAGONAL_DIRECTIONS[1].0, j + DIAGONAL_DIRECTIONS[1].1);
        let (ldi, ldj) = (i + DIAGONAL_DIRECTIONS[2].0, j + DIAGONAL_DIRECTIONS[2].1);
        let (rdi, rdj) = (i + DIAGONAL_DIRECTIONS[3].0, j + DIAGONAL_DIRECTIONS[3].1);

        if lui < 0
            || lui >= parsed_input.len() as isize
            || luj < 0
            || luj >= parsed_input[lui as usize].chars().count() as isize
        {
            return false;
        }

        if rui < 0
            || rui >= parsed_input.len() as isize
            || ruj < 0
            || ruj >= parsed_input[rui as usize].chars().count() as isize
        {
            return false;
        }

        if ldi < 0
            || ldi >= parsed_input.len() as isize
            || ldj < 0
            || ldj >= parsed_input[ldi as usize].chars().count() as isize
        {
            return false;
        }

        if rdi < 0
            || rdi >= parsed_input.len() as isize
            || rdj < 0
            || rdj >= parsed_input[rdi as usize].chars().count() as isize
        {
            return false;
        }

        let da_char_at_lu = parsed_input[lui as usize]
            .chars()
            .nth(luj as usize)
            .unwrap();
        let da_char_at_ru = parsed_input[rui as usize]
            .chars()
            .nth(ruj as usize)
            .unwrap();
        let da_char_at_ld = parsed_input[ldi as usize]
            .chars()
            .nth(ldj as usize)
            .unwrap();
        let da_char_at_rd = parsed_input[rdi as usize]
            .chars()
            .nth(rdj as usize)
            .unwrap();

        let first_diagonal = (da_char_at_lu == 'S' && da_char_at_rd == 'M')
            || (da_char_at_lu == 'M' && da_char_at_rd == 'S');
        let second_diagonal = (da_char_at_ld == 'S' && da_char_at_ru == 'M')
            || (da_char_at_ld == 'M' && da_char_at_ru == 'S');

        first_diagonal && second_diagonal
    }

    fn solve_part2(input: &str) -> u64 {
        let parsed_input = input.split_whitespace().collect::<Vec<&str>>();
        let mut xmas_count = 0;

        for i in 0..parsed_input.len() {
            for (j, char) in parsed_input[i].chars().enumerate() {
                if char == 'A' {
                    if check_for_xmas_part2((i as isize, j as isize), &parsed_input) {
                        xmas_count += 1;
                    }
                }
            }
        }

        xmas_count
    }

    // Your solution here...
    let sol1: u64 = solve_part1(&input);
    let sol2: u64 = solve_part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}
