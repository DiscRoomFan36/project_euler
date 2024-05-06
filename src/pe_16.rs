// Power Digit Sum: https://projecteuler.net/problem=16

type Int = u32;

fn mult_string_number(string_number: &str, to_mult: Int) -> String {
    let mut new_number = String::new();

    let mut carry = string_number
        .as_bytes()
        .iter()
        .rev()
        .fold(0, |carry, char| {
            let digit = char - b'0';

            let carry = digit as Int * to_mult + carry;

            let (carry, next_digit) = (carry / 10, (carry % 10) as u8);

            new_number.push((next_digit + b'0') as char);
            carry
        });

    while carry > 0 {
        let (to_carry, next_digit) = (carry / 10, carry as u8);

        new_number.push((next_digit + b'0') as char);
        carry = to_carry;
    }

    new_number.chars().rev().collect()
}

fn solve_problem(pow2: Int) -> Int {
    let mut string = "1".to_string();

    for _ in 0..pow2 {
        string = mult_string_number(&string, 2);
    }

    string
        .as_bytes()
        .iter()
        .map(|char| (char - b'0') as Int)
        .sum()
}

pub fn main() -> Int {
    solve_problem(1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(solve_problem(15), 26)
    }
}
