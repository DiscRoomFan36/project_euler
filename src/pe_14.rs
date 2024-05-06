// Longest Collatz Sequence: https://projecteuler.net/problem=14

type Int = u64;

fn get_collatz_length(mut n: Int) -> Int {
    let mut count = 1;
    while n != 1 {
        if n % 2 == 1 {
            n = (3 * n + 1) / 2;
            count += 2;
        }
        while n % 2 == 0 {
            n /= 2;
            count += 1;
        }
    }
    count
}

fn solve_problem(max_starting: Int) -> Int {
    (max_starting / 2..max_starting)
        .map(|i| (i, get_collatz_length(i)))
        .max_by_key(|(_i, l)| *l)
        .map(|(i, _l)| i)
        .unwrap()
}

pub fn main() -> Int {
    solve_problem(1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(solve_problem(20), 19)
    }
}
