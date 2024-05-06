// Number Letter Counts: https://projecteuler.net/problem=17

type Int = u32;

const NUMBERS: &[&'static str] = &[
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS_NUMBERS: &[&'static str] = &[
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

fn number_to_string(mut number: usize) -> String {
    if number == 1000 {
        return "one thousand".to_string();
    }

    let mut new_string = String::new();

    if number >= 100 {
        new_string.push_str(&format!("{} hundred", NUMBERS[number / 100]));

        if number % 100 == 0 {
            return new_string;
        }

        new_string.push_str(" and ");

        number %= 100;
    }

    if number < 20 {
        new_string.push_str(NUMBERS[number]);
        return new_string;
    }

    let (tens, digits) = (number / 10, number % 10);

    new_string.push_str(TENS_NUMBERS[tens - 2]);

    if digits != 0 {
        new_string.push_str(&format!("-{}", NUMBERS[digits]));
    }

    new_string
}

fn solve_problem(max_number: usize) -> Int {
    let mut total = 0;
    for num in 1..=max_number {
        total += number_to_string(num).bytes().fold(0, |total, char| {
            if char == b' ' || char == b'-' {
                total
            } else {
                total + 1
            }
        });
    }
    total
}

pub fn main() -> Int {
    solve_problem(1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(solve_problem(5), 19)
    }
}
