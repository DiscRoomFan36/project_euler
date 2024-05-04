// Largest Prime Factor: https://projecteuler.net/problem=3

type Int = u64;

fn solve_problem(mut prime: Int) -> Int {
    let mut largest = 1;

    let mut checking = 2;
    while checking <= prime {
        while prime % checking == 0 {
            prime /= checking;
            largest = checking;
        }

        checking += 1;
    }

    return largest;
}

pub fn main() -> Int {
    solve_problem(600_851_475_143)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(solve_problem(13195), 29)
    }
}
