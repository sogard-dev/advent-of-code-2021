use std::{collections::HashMap, str::FromStr};

pub fn main() {
    println!("Day");
}

type Input = (String, Vec<(String, String)>);

fn puzzle_1(steps: usize, input: Input) -> usize {
    let mut polymer = input.0;

    let mut pattern_map = HashMap::new();
    for (a, b) in input.1 {
        pattern_map.insert(a, b);
    }

    let mut polymer_pairs = HashMap::new();
    for i in (0..polymer.len() - 1).rev() {
        let pattern = &polymer[i..i + 2];
        *polymer_pairs.entry(pattern.to_string()).or_insert_with(|| 0 as usize) += 1;
    }

    for step in 1..=steps {
        let mut new_polymer_pairs = HashMap::new();

        polymer_pairs.iter().for_each(|(k, v)| {
            if let Some(new) = pattern_map.get(k) {
                let first = format!("{}{}", &k[0..1], new);
                let second = format!("{}{}", new, &k[1..2]);

                *new_polymer_pairs.entry(first).or_insert_with(|| 0) += *v;
                *new_polymer_pairs.entry(second).or_insert_with(|| 0) += *v;
            } else {
                *new_polymer_pairs.entry(k.to_string()).or_insert_with(|| 0) += *v;
            }
        });

        polymer_pairs = new_polymer_pairs;
    }


    let mut polymer_count = HashMap::new();
    polymer_pairs.iter().for_each(|(k, v)| {
        k.chars().for_each(|c| {
            *polymer_count.entry(c.clone()).or_insert_with(|| 0) += v;
        })
    });

    *polymer_count.entry(polymer.chars().next().unwrap()).or_insert_with(|| 0) += 1;
    *polymer_count.entry(polymer.chars().rev().next().unwrap()).or_insert_with(|| 0) += 1;

    let max = polymer_count.iter().max_by_key(|(k, v)| *v).unwrap().1 / 2;
    let min = polymer_count.iter().min_by_key(|(k, v)| *v).unwrap().1 / 2;
    println!("  Max: {}", max);
    println!("  Min: {}", min);

    max - min
}

//#[derive(Debug, PartialEq, Clone, Eq, Hash)]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1588, puzzle_1(10, parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(3408, puzzle_1(10, parse(include_str!("puzzle_1.txt"))));

        assert_eq!(2188189693529, puzzle_1(40, parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(3724343376942, puzzle_1(40, parse(include_str!("puzzle_1.txt"))));
    }

    fn parse(s: &str) -> Input {
        let spl: Vec<&str> = s.split("\r\n\r\n").collect();

        let vec = spl[1]
            .lines()
            .map(|line| {
                let lsp: Vec<&str> = line.split(" ").collect();
                (lsp[0].to_string(), lsp[2].to_string())
            })
            .collect();

        (spl[0].to_string(), vec)
    }
}
