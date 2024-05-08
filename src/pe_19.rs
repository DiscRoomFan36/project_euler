// Counting Sundays: https://projecteuler.net/problem=19

type Int = u32;

const DAYS_IN_MONTH: [Int; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn get_days_in_month(month_index: usize, year: Int) -> Int {
    if (month_index == 1) && (year % 4 == 0) {
        DAYS_IN_MONTH[month_index] + 1
    } else {
        DAYS_IN_MONTH[month_index]
    }
}

fn solve_problem() -> Int {
    let mut day_of_week = 1;
    let mut day = 0;
    let mut month_index = 0;
    let mut sunday_count = 0;

    let mut year = 1901;
    loop {
        day = (day + 1) % get_days_in_month(month_index, year);
        day_of_week += 1;

        if day == 0 {
            month_index = (month_index + 1) % DAYS_IN_MONTH.len();
            if month_index == 0 {
                year += 1;
                if year >= 2001 {
                    return sunday_count - 1;
                }
            }
            if day_of_week % 7 == 0 {
                sunday_count += 1;
            }
        }
    }
}

pub fn main() -> Int {
    solve_problem()
}
