pub fn main() {
    println!("Day");
}

type Input = Vec<isize>;


fn puzzle_1(input: Input, days: usize) -> isize {
    let mut prev_vec = input;

    for day in 1..=days {
        let mut new_vec = vec![0,0,0,0,0,0,0,0,0];

        new_vec[0] = prev_vec[1];
        new_vec[1] = prev_vec[2];
        new_vec[2] = prev_vec[3];
        new_vec[3] = prev_vec[4];
        new_vec[4] = prev_vec[5];
        new_vec[5] = prev_vec[6];
        new_vec[6] = prev_vec[7] + prev_vec[0];
        new_vec[7] = prev_vec[8];
        new_vec[8] = prev_vec[0];
        
        prev_vec = new_vec;

    }
    prev_vec.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::util::parse_numbers;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(26, puzzle_1(parse(include_str!("puzzle_1_test.txt")), 18));
        assert_eq!(5934, puzzle_1(parse(include_str!("puzzle_1_test.txt")), 80));
        assert_eq!(365131, puzzle_1(parse(include_str!("puzzle_1.txt")), 80));

        assert_eq!(26984457539, puzzle_1(parse(include_str!("puzzle_1_test.txt")), 256));
        assert_eq!(1650309278600, puzzle_1(parse(include_str!("puzzle_1.txt")), 256));
    }

    fn parse(s: &str) -> Input {
        let mut vec = vec![0,0,0,0,0,0,0,0, 0];
        for n in parse_numbers(s) {
            vec[n as usize] += 1;
        }
        vec
    }
}
