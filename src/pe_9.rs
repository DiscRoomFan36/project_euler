// Special Pythagorean Triplet: https://projecteuler.net/problem=9

type Int = u32;

fn solve_problem(target: Int) -> Int {
    for c in target / 3..target {
        for b in c / 2..c {
            let a = target - (b + c);
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }

    unreachable!("There must be a solution")
}

pub fn main() -> Int {
    solve_problem(1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(solve_problem(3 + 4 + 5), 60)
    }
}
