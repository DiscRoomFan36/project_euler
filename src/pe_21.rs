// Amicable Numbers: https://projecteuler.net/problem=21

type Int = u32;

fn get_proper_sum(num: Int) -> Int {
    if num == 1 {
        return 1;
    }

    let mut total = 1;

    let mut checking = 2;
    while checking * checking < num {
        if num % checking == 0 {
            total += checking + (num / checking);
        }
        checking += 1;
    }

    if checking * checking == num {
        total += checking;
    }

    total
}

fn solve_problem(max: Int) -> Int {
    let mut total = 0;

    for a in 1..max {
        let b = get_proper_sum(a);
        if a != b && a == get_proper_sum(b) {
            total += a;
        }
    }

    total
}

pub fn main() -> Int {
    solve_problem(10_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(get_proper_sum(220), 284);
        assert_eq!(get_proper_sum(284), 220);
    }
}
