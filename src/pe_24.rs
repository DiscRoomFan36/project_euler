// Lexicographic Permutations: https://projecteuler.net/problem=24

type Int = u64;

struct Permutations<T> {
    stack: Vec<(Vec<T>, Vec<T>)>,
}

impl<T> Permutations<T> {
    fn new(objects: Vec<T>) -> Self {
        Self {
            stack: vec![(vec![], objects)],
        }
    }
}

impl<T> Iterator for Permutations<T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((current, to_add)) = self.stack.pop() {
            if to_add.is_empty() {
                return Some(current);
            }

            for index in (0..to_add.len()).rev() {
                let mut current = current.clone();
                let mut to_add = to_add.clone();
                current.push(to_add.remove(index));
                self.stack.push((current, to_add))
            }
        }
        return None;
    }
}

fn solve_problem(num_objects: u8, ith_permutation: usize) -> Int {
    Permutations::new((0..num_objects).collect())
        .nth(ith_permutation - 1)
        .unwrap()
        .iter()
        .fold(0, |z, u| (z * 10) + (*u as Int))
}

pub fn main() -> Int {
    solve_problem(10, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_test() {
        assert_eq!(solve_problem(3, 6), 210)
    }
}
