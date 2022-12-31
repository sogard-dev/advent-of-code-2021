use crate::grid::Grid;

pub fn main() {
    println!("Day");
}

type Input = Grid<isize>;

fn puzzle_1(input: Input) -> isize {
    let start = (0, 0);
    let end = (input.width - 1, input.height - 1);

    let mut cost_at_end = 0;
    input.bfs_with_cost(
        &start,
        |pos, cost| {
            if end.eq(pos) {
                cost_at_end = cost;
            }
        },
        |p, m| *m,
    );

    cost_at_end
}

//#[derive(Debug, PartialEq, Clone, Eq, Hash)]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(40, puzzle_1(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(523, puzzle_1(parse(include_str!("puzzle_1.txt"))));

        assert_eq!(315, puzzle_1(parse_2(include_str!("puzzle_1_test.txt"))));
        assert_eq!(1, puzzle_1(parse_2(include_str!("puzzle_1.txt"))));
    }

    fn parse(s: &str) -> Input {
        let mut grid = Grid::new(s, |_, c| c.to_digit(10).unwrap() as isize);
        let mut connections = vec![];

        grid.for_every_delta(
            |a, _, b, _| {
                connections.push((*a, *b));
            },
            vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
        );

        grid.add_connections(connections);

        grid
    }

    fn parse_2(s: &str) -> Input {
        let org_lines: Vec<&str> = s.lines().collect();

        let mult = 5;

        let mut vec_lines = vec![];
        let org_size = s.lines().count();
        let size = org_size * mult;
        for row in 0..size {
            let mut str = String::new();
            for c in 0..size {
                str.push(' ');
            }
            vec_lines.push(str);
        }

        for r in 0..org_size {
            for (c, ch) in org_lines[r].chars().enumerate() {
                let cost = ch.to_digit(10).unwrap() as usize;

                for rx in 0..mult {
                    for cx in 0..mult {

                        let new_cost = ((rx + cx + cost - 1) % 9) + 1;
                        let r_m = r + rx * org_size;
                        let c_m = c + cx * org_size;
                        vec_lines[r_m].replace_range(c_m..c_m + 1, &new_cost.to_string());
                    }
                }
            }
        }

        let s = &vec_lines.join("\r\n");

        parse(s)
    }
}
