use itertools::Itertools;

pub fn main() {
    println!("Day");
}

type Input = Vec<Vec<u32>>;

fn count_for_index(input: &Input, index: usize) -> i32 {
    let mut zero_count = 0;
    let mut one_count = 0;

    for line in input.iter() {
        if line[index] == 0 {
            zero_count += 1;
        } else {
            one_count += 1;
        }
    }

    one_count - zero_count
}

fn puzzle_1(input: Input) -> u32 {
    let mut gamma = String::new();
    let mut epsilon = String::new();

    for index in 0..input[0].len() {
        let extra_ones = count_for_index(&input, index);
        if extra_ones > 0 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    u32::from_str_radix(&gamma, 2).unwrap() * u32::from_str_radix(&epsilon, 2).unwrap()
}

fn puzzle_2(input: Input) -> u32 {
    let mut oxygen = input.clone();
    let mut scrubber = input.clone();

    for index in 0..input[0].len() {
        if oxygen.len() > 1 {
            let extra_ones = count_for_index(&oxygen, index);
            if extra_ones >= 0 {
                oxygen.retain(|e| e[index] == 1);
            } else {
                oxygen.retain(|e| e[index] == 0);
            }
        }

        if scrubber.len() > 1 {
            let extra_ones = count_for_index(&scrubber, index);
            if extra_ones >= 0 {
                scrubber.retain(|e| e[index] == 0);
            } else {
                scrubber.retain(|e| e[index] == 1);
            }
        }
    }


    let oxy_str = oxygen[0].clone().into_iter().join("");
    let scrub_str = scrubber[0].clone().into_iter().join("");

    u32::from_str_radix(&oxy_str, 2).unwrap() * u32::from_str_radix(&scrub_str, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(198, puzzle_1(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(3969000, puzzle_1(parse(include_str!("puzzle_1.txt"))));

        assert_eq!(230, puzzle_2(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(4267809, puzzle_2(parse(include_str!("puzzle_1.txt"))));
    }

    fn parse(s: &str) -> Input {
        s.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect()
    }
}
