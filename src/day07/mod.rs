use std::{borrow::Borrow, collections::HashMap};

pub fn main() {
    println!("Day");
}

type Input = Vec<isize>;

fn puzzle_1(input: Vec<isize>) -> isize {
    let max = *input.iter().max().unwrap();
    let min = *input.iter().min().unwrap();

    let mut best = isize::MAX;

    for i in min..=max {
        let mut fuel = 0;
        for x in &input {
            fuel += (i - x).abs()
        }

        best = best.min(fuel)
    }

    best
}

fn puzzle_2(input: Input) -> isize {
    let max = *input.iter().max().unwrap();
    let min = *input.iter().min().unwrap();

    let mut cache: HashMap<isize, isize> = HashMap::new();

    (min..=max)
        .map(|i| {
            input
                .iter()
                .map(|x| {
                    let distance = (i - x).abs();
                    *cache.entry(distance).or_insert_with(|| (1..=distance).sum())
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(37, puzzle_1(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(341534, puzzle_1(parse(include_str!("puzzle_1.txt"))));

        assert_eq!(168, puzzle_2(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(93397632, puzzle_2(parse(include_str!("puzzle_1.txt"))));
    }

    fn parse(s: &str) -> Input {
        util::parse_numbers(s)
    }
}
