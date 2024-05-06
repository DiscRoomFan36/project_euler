// Maximum Path Sum I: https://projecteuler.net/problem=18

use std::cmp::max;

type Int = u32;

const INPUT: &str = "75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

fn solve_problem(input_string: &str) -> Int {
    let numbers: Vec<Vec<Int>> = input_string
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|str| str.parse().expect("contains all numbers"))
                .collect()
        })
        .collect();

    let mut dynamic: Vec<Vec<Int>> = Vec::new();
    for i in 1..=numbers.len() {
        dynamic.push(vec![0; i])
    }

    let last_index = dynamic.len() - 1;
    for i in 0..dynamic.len() {
        dynamic[last_index][i] = numbers[last_index][i];
    }

    for col_index in (0..dynamic.len() - 1).rev() {
        for row_index in 0..=col_index {
            dynamic[col_index][row_index] = max(
                dynamic[col_index + 1][row_index],
                dynamic[col_index + 1][row_index + 1],
            ) + numbers[col_index][row_index];
        }
    }

    dynamic[0][0]
}

pub fn main() -> Int {
    solve_problem(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3
7 4
2 4 6
8 5 9 3";

    #[test]
    fn solves_test() {
        assert_eq!(solve_problem(TEST_INPUT), 23)
    }
}
