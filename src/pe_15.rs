// Lattice Paths: https://projecteuler.net/problem=15

type Int = u64;

fn solve_problem(grid_size: usize) -> Int {
    let size = grid_size + 1;

    let mut dynamic = vec![0; size * size];
    for x in 0..size {
        dynamic[x] = 1;
        dynamic[x * size] = 1;
    }

    for x in 1..size {
        for y in 1..size {
            dynamic[x * size + y] = dynamic[(x - 1) * size + y] + dynamic[x * size + (y - 1)]
        }
    }

    return dynamic[size * size - 1];
}

pub fn main() -> Int {
    solve_problem(20)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(solve_problem(2), 6)
    }
}
