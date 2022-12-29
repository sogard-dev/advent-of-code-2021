use crate::grid::Grid;

pub fn main() {
    println!("Day");
}

type Input = Grid<u32>;

fn puzzle_1(mut input: Input) -> usize {
    let mut positions = vec![];
    input.for_every(|pos, _| positions.push(*pos));

    let mut count_flashes = 0;

    println!("Before any steps:");
    input.print(|pos, n| format!("{}", n));
    for step in 1..=100 {
        let mut flashes = vec![];
        for pos in positions.iter() {
            *input.get_model_mut(pos).unwrap() += 1;
            if *input.get_model(pos).unwrap() == 10 {
                flashes.push(*pos);
            }
        }

        count_flashes += flashes.len();

        while let Some(pos) = flashes.pop() {
            let mut flash_from_here = vec![];
            input.for_every_neighbour(
                &pos,
                |n, _| {
                    flash_from_here.push(*n);
                },
                vec![(0, -1), (0, 1), (1, -1), (1, 0), (1, 1), (-1, -1), (-1, 0), (-1, 1)],
            );

            for p_n in flash_from_here {
                *input.get_model_mut(&p_n).unwrap() += 1;
                if *input.get_model(&p_n).unwrap() == 10 {
                    flashes.push(p_n);
                    count_flashes += 1;
                }
            }
        }

        for pos in positions.iter() {
            if *input.get_model(pos).unwrap() >= 10 {
                *input.get_model_mut(pos).unwrap() = 0;
            }
        }

        println!("After step {}:", step);
        input.print(|pos, n| format!("{}", n));
    }

    count_flashes
}

fn puzzle_2(mut input: Input) -> usize {
    let mut positions = vec![];
    input.for_every(|pos, _| positions.push(*pos));

    println!("Before any steps:");
    input.print(|pos, n| format!("{}", n));
    for step in 1..=10000 {
        let mut flashes_per_step = 0;
        let mut flashes = vec![];
        for pos in positions.iter() {
            *input.get_model_mut(pos).unwrap() += 1;
            if *input.get_model(pos).unwrap() == 10 {
                flashes.push(*pos);
            }
        }

        flashes_per_step += flashes.len();

        while let Some(pos) = flashes.pop() {
            let mut flash_from_here = vec![];
            input.for_every_neighbour(
                &pos,
                |n, _| {
                    flash_from_here.push(*n);
                },
                vec![(0, -1), (0, 1), (1, -1), (1, 0), (1, 1), (-1, -1), (-1, 0), (-1, 1)],
            );

            for p_n in flash_from_here {
                *input.get_model_mut(&p_n).unwrap() += 1;
                if *input.get_model(&p_n).unwrap() == 10 {
                    flashes.push(p_n);
                    flashes_per_step += 1;
                }
            }
        }

        if flashes_per_step == positions.len() {
            return step;
        }

        for pos in positions.iter() {
            if *input.get_model(pos).unwrap() >= 10 {
                *input.get_model_mut(pos).unwrap() = 0;
            }
        }

        println!("After step {}:", step);
        input.print(|pos, n| format!("{}", n));
    }

    panic!("Did not find");
}

//#[derive(Debug, PartialEq, Clone, Eq, Hash)]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1656, puzzle_1(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(1617, puzzle_1(parse(include_str!("puzzle_1.txt"))));

        assert_eq!(195, puzzle_2(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(258, puzzle_2(parse(include_str!("puzzle_1.txt"))));
    }

    fn parse(s: &str) -> Input {
        Grid::new(s, |_, n| n.to_digit(10).unwrap())
    }
}
