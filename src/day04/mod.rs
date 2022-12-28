use std::collections::HashSet;

pub fn main() {
    println!("Day");
}

type Input = (Vec<Board>, Vec<isize>);

fn get_sum(board: &Board) -> isize {
    println!("Sum of board: {:?}", board);

    let mut numbers = HashSet::new();
    let mut sum = 0;
    for o in &board.options {
        for n in o {
            numbers.insert(*n);
        }
    }

    println!("Unique numbers: {:?}", numbers);

    numbers.iter().sum()
}

fn puzzle_1(input: Input) -> isize {
    let mut boards = input.0;
    let numbers = input.1;

    let mut round = 1;
    for num in numbers {
        println!(" Drawing {} in round {}", num, round);

        for mut board in &mut boards {
            for mut option in &mut board.options {
                if option.remove(&num) {
                    println!("Removing {} from board option, remaining: {}", num, option.len());
                }
            }

            for mut option in &board.options {
                if option.len() == 0 {
                    return get_sum(board) * num;
                }
            }
        }

        round += 1;
    }
    0
}

fn puzzle_2(input: Input) -> isize {
    let mut boards = input.0;
    let numbers = input.1;

    let mut round = 1;
    for num in numbers {
        println!(" Drawing {} in round {}", num, round);

        for mut board in &mut boards {
            for mut option in &mut board.options {
                if option.remove(&num) {
                    println!("Removing {} from board option, remaining: {}", num, option.len());
                }
            }
        }

        for (idx, board) in boards.clone().into_iter().enumerate().rev() {
            for mut option in &board.options {
                if option.len() == 0 {
                    if boards.len() == 1 {
                        return get_sum(&boards[0]) * num;
                    }

                    boards.remove(idx);
                    break;
                }
            }
        }

        round += 1;
    }
    0
}

#[derive(Debug, PartialEq, Clone, Eq)]
struct Board {
    options: Vec<HashSet<isize>>,
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::util;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4512, puzzle_1(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(50008, puzzle_1(parse(include_str!("puzzle_1.txt"))));

        assert_eq!(1924, puzzle_2(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(17408, puzzle_2(parse(include_str!("puzzle_1.txt")))); //NOT
    }

    fn parse(s: &str) -> Input {
        let spl: Vec<&str> = s.split("\r\n\r\n").collect();

        let mut boards = vec![];
        let mut numbers = vec![];

        let mut iter = spl.into_iter();

        numbers = util::parse_numbers(iter.next().unwrap());
        while let Some(board_str) = iter.next() {
            let mut rows = vec![HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new()];
            let mut columns = vec![HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new()];

            for (row, line) in board_str.lines().enumerate() {
                for (column, number) in util::parse_numbers(line).into_iter().enumerate() {
                    rows[row].insert(number);
                    columns[column].insert(number);
                }
            }

            rows.append(&mut columns);

            let board = Board { options: rows };
            boards.push(board);
        }

        (boards, numbers)
    }
}
