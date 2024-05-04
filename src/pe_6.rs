// Sum Square Difference: https://projecteuler.net/problem=6

type Int = u32;

fn solve_problem(max: Int) -> Int {
    let mut sum_squares = 0;
    let mut sum = 0;
    for n in 1..=max {
        sum_squares += n * n;
        sum += n;
    }
    return (sum * sum) - sum_squares;
}

pub fn main() -> Int {
    solve_problem(100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(solve_problem(10), 2640)
    }
}
