use std::collections::VecDeque;

pub fn main() {
    println!("Day 1");
}

type Input = Vec<isize>;

fn puzzle_1(input: Input) -> usize {
    let mut last_number = None;

    input.iter().fold(0, |acc, v| {
        let mut ret = acc;

        if let Some(prev) = last_number {
            if prev < v {
                ret += 1;
            }
        }
        last_number = Some(v);

        ret
    })
}

fn puzzle_2(input: Input) -> usize {
    let mut window: VecDeque<isize> = VecDeque::new();

    input.iter().fold(0, |acc, v| {
        let mut ret = acc;
        if window.len() == 3 {
            let prev_sum = window.iter().sum::<isize>();
            window.pop_front();
            window.push_back(*v);
            let new_sum = window.iter().sum::<isize>();
            if new_sum > prev_sum {
                ret += 1;
            }
        } else {
            window.push_back(*v);

        }

        
        ret
    })
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(7, puzzle_1(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(1655, puzzle_1(parse(include_str!("puzzle_1.txt"))));

        assert_eq!(5, puzzle_2(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(1683, puzzle_2(parse(include_str!("puzzle_1.txt"))));
    }

    fn parse(s: &str) -> Input {
        s.lines().map(|s| util::parse_numbers(s)[0]).collect()
    }
}
