use std::collections::HashMap;

pub fn main() {
    println!("Day");
}

type Input = Vec<Vec<String>>;

fn puzzle_1(input: Input) -> usize {
    let mut lengths = HashMap::new();
    get_numbers().iter().for_each(|num| *lengths.entry(num.positions.len()).or_insert_with(|| 0) += 1);

    input.iter().map(|vec| vec.iter().skip(10).take(4).map(|s| if *lengths.get(&s.len()).unwrap() == 1 { 1 } else { 0 }).sum::<usize>()).sum()
}

fn puzzle_2(input: Input) -> usize {
    0
}

#[derive(Debug, PartialEq, Clone, Eq)]
enum Position {
    Top,
    Middle,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn get_numbers() -> Vec<Number> {
    vec![
        Number { num: 0, positions: vec![Position::Top, Position::TopRight, Position::TopRight, Position::BottomLeft, Position::BottomRight, Position::Bottom] },
        Number { num: 1, positions: vec![Position::TopRight, Position::BottomRight] },
        Number { num: 2, positions: vec![Position::Top, Position::TopRight, Position::Middle, Position::BottomLeft, Position::Bottom] },
        Number { num: 3, positions: vec![Position::Top, Position::TopRight, Position::Middle, Position::BottomRight, Position::Bottom] },
        Number { num: 4, positions: vec![Position::TopLeft, Position::TopRight, Position::Middle, Position::BottomRight] },
        Number { num: 5, positions: vec![Position::Top, Position::TopLeft, Position::Middle, Position::BottomRight, Position::Bottom] },
        Number { num: 6, positions: vec![Position::Top, Position::TopLeft, Position::Middle, Position::BottomLeft, Position::BottomRight, Position::Bottom] },
        Number { num: 7, positions: vec![Position::Top, Position::TopRight, Position::BottomRight] },
        Number { num: 8, positions: vec![Position::Top, Position::TopRight, Position::TopRight, Position::Middle, Position::BottomLeft, Position::BottomRight, Position::Bottom] },
        Number { num: 9, positions: vec![Position::Top, Position::TopRight, Position::TopRight, Position::Middle, Position::BottomRight, Position::Bottom] },
    ]
}

#[derive(Debug, PartialEq, Clone, Eq)]
struct Number {
    num: usize,
    positions: Vec<Position>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(26, puzzle_1(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(479, puzzle_1(parse(include_str!("puzzle_1.txt"))));

        assert_eq!(61229, puzzle_2(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(1, puzzle_2(parse(include_str!("puzzle_1.txt"))));
    }

    fn parse(s: &str) -> Input {
        s.lines().map(|line| line.replace(" |", "").split(" ").map(|s| s.to_string()).collect()).collect()
    }
}
