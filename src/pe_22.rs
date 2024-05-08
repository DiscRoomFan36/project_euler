// Names Scores: https://projecteuler.net/problem=22

use std::collections::BinaryHeap;
use std::{fs::File, io::Read};

type Int = u32;

#[inline]
fn name_to_number(name: &str) -> Int {
    let mut total = 0;

    for char in name.bytes() {
        total += (char - b'A' + 1) as Int
    }

    total
}

fn solve_problem() -> Int {
    let mut f = File::open("assets/0022_names.txt").expect("File Exists at location");

    let mut all_names = String::new();
    f.read_to_string(&mut all_names).expect("Read it to string");

    let mut names = all_names
        .split(",")
        .map(|string| &string[1..string.len() - 1])
        .collect::<BinaryHeap<&str>>();

    let mut total = 0;

    let mut i = names.len();
    while let Some(name) = names.pop() {
        total += (i as Int) * name_to_number(name);

        i -= 1;
    }

    total
}

pub fn main() -> Int {
    solve_problem()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(name_to_number("COLIN"), 53)
    }
}
