// Smallest Multiple: https://projecteuler.net/problem=5

type Int = u32;

fn lcm(a: Int, b: Int) -> Int {
    let (mut ap, mut bp) = (a, b);
    while bp != 0 {
        (ap, bp) = (bp, ap % bp)
    }
    return (a / ap) * b;
}

fn solve_problem(max: Int) -> Int {
    let mut num = lcm(1, 2);
    for n in 3..=max {
        num = lcm(num, n)
    }
    return num;
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
