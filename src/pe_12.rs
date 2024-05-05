// Highly Divisible Triangular Number: https://projecteuler.net/problem=12

type Int = u32;

// taken from the tread on problem 12, might have to come back and use this
// As n and n+1 have no factors in common (except for the number 1) one can multiply the factor counts in n/2 and n+1, or n and (n+1)/2 as the case may be, to arrive at the factor count of the nth triangle number. This makes things a lot faster. - bartmeijer

fn get_num_divisors(num: Int) -> Int {
    if num == 1 {
        return 1;
    }

    let mut total = 2;

    let mut checking = 2;
    while checking * checking < num {
        if num % checking == 0 {
            total += 2;
        }
        checking += 1;
    }

    if checking * checking == num {
        total + 1
    } else {
        total
    }
}

fn solve_problem(num_divisors: Int) -> Int {
    let mut count = 2;
    let mut triangle_number = 1;
    while get_num_divisors(triangle_number) < num_divisors {
        triangle_number += count;
        count += 1;
    }
    triangle_number
}

pub fn main() -> Int {
    solve_problem(500)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(solve_problem(5), 28)
    }
}
