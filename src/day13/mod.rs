use std::collections::HashSet;

use crate::{
    grid::{self, Coordinate},
    util,
};

pub fn main() {
    println!("Day");
}

type Input = (HashSet<Coordinate>, Vec<Fold>);

fn puzzle_1(input: Input) -> usize {
    fold(input, 1).len()
}

fn puzzle_2(input: Input) -> usize {
    let len = input.1.len();
    let folded = fold(input, len);
    print(&folded);
    folded.len()
}

fn print(input: &HashSet<Coordinate>) {
    let row_min = input.iter().min_by_key(|e| e.0).unwrap().0;
    let row_max = input.iter().max_by_key(|e| e.0).unwrap().0;
    let column_min = input.iter().min_by_key(|e| e.1).unwrap().1;
    let column_max = input.iter().max_by_key(|e| e.1).unwrap().1;

    for r in row_min..=row_max {
        for c in column_min..=column_max {
            print!(
                "{}",
                match input.contains(&(r, c)) {
                    true => '#',
                    false => '.',
                }
            );
        }
        println!("");
    }
    println!("");
}

fn fold(mut input: Input, folds: usize) -> HashSet<Coordinate> {
    let mut dots = input.0;

    for (n, f) in input.1.iter().enumerate() {
        if n >= folds {
            break;
        }

        dots = dots
            .iter()
            .filter_map(|e| {
                let point_in_between = match f {
                    Fold::Row(i) => (*i as isize, e.1),
                    Fold::Column(i) => (e.0, *i as isize),
                };

                match f {
                    Fold::Row(i) => {
                        if e.0 > point_in_between.0 {
                            let shift = (grid::manhatten_distance(e, &point_in_between) * 2) as isize;
                            return Some((e.0 - shift, e.1));
                        } else {
                            return Some(*e);
                        }
                    }
                    Fold::Column(i) => {
                        if e.1 > point_in_between.1 {
                            let shift = (grid::manhatten_distance(e, &point_in_between) * 2) as isize;
                            return Some((e.0, e.1 - shift));
                        } else {
                            return Some(*e);
                        }
                    }
                }

                None
            })
            .collect();
    }

    dots
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum Fold {
    Row(usize),
    Column(usize),
}

#[cfg(test)]
mod tests {

    use crate::util;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(17, puzzle_1(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(592, puzzle_1(parse(include_str!("puzzle_1.txt"))));

        assert_eq!(1, puzzle_2(parse(include_str!("puzzle_1.txt"))));
    }

    fn parse(s: &str) -> Input {
        let mut dots = HashSet::new();
        let mut folds = vec![];

        s.lines().for_each(|line| {
            let numbers = util::parse_numbers(line);
            if numbers.len() == 2 {
                dots.insert((numbers[1], numbers[0]));
            } else if numbers.len() == 1 {
                if line.contains("fold along y") {
                    folds.push(Fold::Row(numbers[0] as usize))
                } else {
                    folds.push(Fold::Column(numbers[0] as usize))
                }
            }
        });

        (dots, folds)
    }
}
