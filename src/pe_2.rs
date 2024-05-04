// Even Fibonacci Numbers: https://projecteuler.net/problem=2

fn solve_problem(max: u32) -> u32 {
    let mut a = 1;
    let mut b = 2;

    let mut total = 0;
    while b < max {
        let tmp = a + b;
        a = b;
        b = tmp;

        if a % 2 == 0 {
            total += a;
        }
    }
    return total;
}

pub fn main() -> u32 {
    solve_problem(4_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(solve_problem(100), 2 + 8 + 34)
    }
}
