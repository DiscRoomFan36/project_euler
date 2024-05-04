// 10001st Prime: https://projecteuler.net/problem=7

type Int = u32;

fn solve_problem(prime: Int) -> Int {
    let prime = prime as usize;

    let n = prime as f32;
    let limit = (n * (n.ln() + n.ln().ln())) as usize + 10;

    let mut sieve_list = vec![true; limit];

    for i in 2..(limit as f32).sqrt() as usize {
        if sieve_list[i - 2] {
            for j in (i * i..limit).step_by(i) {
                sieve_list[j - 2] = false
            }
        }
    }

    sieve_list
        .iter()
        .enumerate()
        .filter_map(|(n, b)| (*b).then_some(n + 2))
        .take(prime)
        .last()
        .unwrap() as Int
}

pub fn main() -> Int {
    solve_problem(10_001)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(solve_problem(6), 13)
    }
}
