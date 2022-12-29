use crate::grid;

pub fn main() {
    println!("Day");
}

type Input = grid::Grid<u32>;

fn puzzle_1(input: Input) -> u32 {
    let mut result = 0;

    input.for_every(|pos, value| {
        let mut is_lowest = true;
        input.for_every_neighbour(pos, |_, n| is_lowest = is_lowest && value < n, vec![(1, 0), (-1, 0), (0, 1), (0, -1)]);

        if is_lowest {
            result += 1 + value;
        }
    });

    result
}

fn puzzle_2(mut input: Input) -> usize {
    let mut lowest = vec![];
    input.for_every(|pos, value| {
        let mut is_lowest = true;
        input.for_every_neighbour(pos, |_, n| is_lowest = is_lowest && value < n, vec![(1, 0), (-1, 0), (0, 1), (0, -1)]);

        if is_lowest {
            lowest.push(*pos);
        }
    });

    let mut connections = vec![];
    input.for_every(|pos, value| {
        if *value != 9 {
            input.for_every_neighbour(
                pos,
                |other_pos, n| {
                    if *n != 9 {
                        connections.push((*pos, *other_pos));
                    }
                },
                vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
            );
        }
    });

    input.add_connections(connections);

    let mut biggest = vec![];
    for pos in lowest {
        let size = input.can_visit(&pos);
        println!("Basin: {:?}  size: {}", pos, size);
        biggest.push(size);
    }
    biggest.sort();

    biggest.iter().rev().take(3).fold(1, |acc, v| acc * v)
}

#[cfg(test)]
mod tests {
    use crate::grid::Grid;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(15, puzzle_1(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(588, puzzle_1(parse(include_str!("puzzle_1.txt"))));

        assert_eq!(1134, puzzle_2(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(964712, puzzle_2(parse(include_str!("puzzle_1.txt"))));
    }

    fn parse(s: &str) -> Input {
        Grid::new(s, |p, s| s.to_digit(10).unwrap())
    }
}
