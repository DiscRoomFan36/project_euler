// Largest Palindrome Product: https://projecteuler.net/problem=4

type Int = u32;

fn is_palindrome(num: Int) -> bool {
    let mut pal = 0;
    let mut temp = num;

    while temp != 0 {
        pal = (pal * 10) + (temp % 10);
        temp /= 10;
    }

    return pal == num;
}

fn solve_problem(digits: Int) -> Int {
    let max_num = 10_u32.pow(digits);

    // can multiply py large number less than 10,
    // to cut down on search space, and most
    // likely the larges palindromes will be
    // within the first set of digits
    let min_num = 10_u32.pow(digits - 1) * 9;

    let mut largest = 0;

    for x in (min_num..max_num).rev() {
        for y in (x..max_num).rev() {
            let checking = x * y;
            if checking > largest {
                if is_palindrome(checking) {
                    largest = checking;
                }
            }
        }
    }

    return largest;
}

pub fn main() -> Int {
    solve_problem(3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(solve_problem(2), 9009)
    }
}
