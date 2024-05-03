// Multiples of 3 or 5

fn get_multiples_3_5(max: u32) -> u32 {
    (3..max).filter(|n| (n % 3 == 0) || (n % 5 == 0)).sum()
}

pub fn main() -> u32 {
    get_multiples_3_5(1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(get_multiples_3_5(10), 23)
    }
}
