// Summation of Primes: https://projecteuler.net/problem=10

type Int = u64;

fn solve_problem(max_num: Int) -> u64 {
    let limit = max_num as usize;

    let mut sieve_list = vec![true; limit];

    for i in 2..(limit as f32).sqrt() as usize + 1 {
        if sieve_list[i - 2] {
            for j in (i * i..limit).step_by(i) {
                sieve_list[j - 2] = false
            }
        }
    }

    (sieve_list
        .iter()
        .enumerate()
        .filter_map(|(n, b)| (*b).then_some(n + 2))
        .sum::<usize>()
        - (limit * 2 + 1)) as Int // remove over-counting
}

pub fn main() -> u64 {
    solve_problem(2_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(solve_problem(10), 17)
    }
}
