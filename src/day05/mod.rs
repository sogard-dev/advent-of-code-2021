use std::collections::HashMap;

use crate::grid::{Coordinate, Segment};

pub fn main() {
    println!("Day");
}

type Input = Vec<Segment>;

fn puzzle_1(input: Input) -> usize {
    let mut map = HashMap::new();

    for segment in &input {
        if segment.from.0 == segment.to.0 || segment.from.1 == segment.to.1 {
            for coord in segment.iter() {
                *map.entry(coord).or_insert_with(|| 0) += 1;
            }
        }
    }

    map.iter().filter(|e| *e.1 > 1).count()
}

fn puzzle_2(input: Input) -> usize {
    let mut map = HashMap::new();

    for segment in &input {
        for coord in segment.iter() {
            *map.entry(coord).or_insert_with(|| 0) += 1;
        }
    }

    map.iter().filter(|e| *e.1 > 1).count()
}

#[cfg(test)]
mod tests {
    use crate::{grid::Segment, util};

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(5, puzzle_1(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(6005, puzzle_1(parse(include_str!("puzzle_1.txt"))));

        assert_eq!(12, puzzle_2(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(23864, puzzle_2(parse(include_str!("puzzle_1.txt"))));
    }

    fn parse(s: &str) -> Input {
        s.lines().filter_map(|line| if let [a, b, c, d] = util::parse_numbers(line)[..] { Some(Segment { from: (a, b), to: (c, d) }) } else { None }).collect()
    }
}
