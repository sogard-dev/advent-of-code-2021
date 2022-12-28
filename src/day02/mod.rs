pub fn main() {
    println!("Day 2");
}

type Input = Vec<Command>;

fn puzzle_1(input: Input) -> usize {
    let mut depth = 0;
    let mut x = 0;

    for command in input {
        match command {
            Command::Forward(n) => x += n,
            Command::Up(n) => depth -= n,
            Command::Down(n) => depth += n,
        }
    }

    depth * x
}

fn puzzle_2(input: Input) -> usize {
    let mut aim = 0;
    let mut depth = 0;
    let mut x = 0;

    for command in input {
        match command {
            Command::Forward(n) => {
                x += n;
                depth += aim * n;
            },
            Command::Up(n) => aim -= n,
            Command::Down(n) => aim += n,
        }
    }

    depth * x
}

#[derive(Debug, PartialEq, Clone, Eq)]
enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::util;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(150, puzzle_1(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(2147104, puzzle_1(parse(include_str!("puzzle_1.txt"))));

        assert_eq!(900, puzzle_2(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(2044620088, puzzle_2(parse(include_str!("puzzle_1.txt"))));
    }

    fn parse(s: &str) -> Input {
        s.lines()
            .map(|line| {
                let spl: Vec<&str> = line.split(" ").collect();
                let num = util::parse_numbers(spl[1])[0];

                match spl[0] {
                    "forward" => Command::Forward(num as usize),
                    "up" => Command::Up(num as usize),
                    "down" => Command::Down(num as usize),
                    _ => panic!("Unknown command: {}", spl[0]),
                }
            })
            .collect()
    }
}
