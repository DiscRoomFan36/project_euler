// 1000-digit Fibonacci Number: https://projecteuler.net/problem=25

use std::cmp::max;

type Int = u32;

fn add_string_numbers(string_number: &str, to_add: &str) -> String {
    let mut digits = Vec::with_capacity(max(string_number.len(), to_add.len()) + 1);
    digits.push(0);

    let mut iters = Vec::new();
    for string in [string_number, to_add] {
        iters.push(string.bytes().rev().peekable())
    }

    while iters.iter_mut().any(|iter| iter.peek().is_some()) {
        let mut accumulator = digits.pop().unwrap();

        for iter in iters.iter_mut() {
            accumulator += iter.next().unwrap_or(b'0') - b'0'
        }

        digits.push(accumulator % 10);
        digits.push(accumulator / 10);
    }

    digits
        .into_iter()
        .rev()
        .skip_while(|digit| *digit == 0)
        .map(|digit| (digit + b'0') as char)
        .collect()
}

fn solve_problem(num_digits: usize) -> Int {
    let mut cur = "1".to_string();
    let mut next = "1".to_string();

    let mut count = 1;

    while cur.len() < num_digits {
        let temp = add_string_numbers(&cur, &next);
        cur = next;
        next = temp;

        count += 1;
    }

    count
}

pub fn main() -> Int {
    solve_problem(1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(solve_problem(3), 12)
    }
}
