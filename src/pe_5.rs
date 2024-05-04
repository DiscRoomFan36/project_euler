// Smallest Multiple: https://projecteuler.net/problem=5

type Int = u32;

fn is_divisible(max: Int, number: Int) -> bool {
    for div in 2..=max {
        if number % div != 0 {
            return false;
        }
    }
    return true;
}

fn solve_problem(max: Int) -> Int {
    let mut checking = max * (max - 1);
    while !is_divisible(max, checking) {
        // skip by max, it must divide by max
        checking += max;
    }
    return checking;
}

pub fn main() -> Int {
    solve_problem(20)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(solve_problem(10), 2520)
    }
}
