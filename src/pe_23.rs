// Non-Abundant Sums: https://projecteuler.net/problem=23

type Int = u32;

fn is_abundant(num: usize) -> bool {
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

    total > num
}

const MAX_NUMBER: usize = 28123;

fn solve_problem() -> Int {
    let abundant_numbers: Vec<usize> = (1..MAX_NUMBER).filter(|num| is_abundant(*num)).collect();

    let mut numbers_vec = vec![true; MAX_NUMBER];

    // this takes the longest
    for i in 0..abundant_numbers.len() {
        let a = abundant_numbers[i];
        for j in 0..=i {
            let sum = a + abundant_numbers[j];
            if sum >= MAX_NUMBER {
                break;
            }
            numbers_vec[sum] = false;
        }
    }

    numbers_vec
        .iter()
        .enumerate()
        .filter_map(|(i, b)| (*b).then_some(i as Int))
        .sum()
}

pub fn main() -> Int {
    solve_problem()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(is_abundant(12), true);
        assert_eq!(is_abundant(13), false);
    }
}
