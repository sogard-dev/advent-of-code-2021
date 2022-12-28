pub fn main() {
    println!("Day");
}

type Input = String;


fn puzzle_1(input: Input) -> usize {
    0
}

fn puzzle_2(input: Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, puzzle_1(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(1, puzzle_1(parse(include_str!("puzzle_1.txt"))));

        assert_eq!(1, puzzle_2(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(1, puzzle_2(parse(include_str!("puzzle_1.txt"))));
    }

    fn parse(s: &str) -> Input {
        s.to_string()
    }
}
