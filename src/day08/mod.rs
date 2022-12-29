use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn main() {
    println!("Day");
}

type Input = Vec<Vec<String>>;

fn puzzle_1(input: Input) -> usize {
    let mut lengths = HashMap::new();
    get_numbers().iter().for_each(|num| *lengths.entry(num.positions.len()).or_insert_with(|| 0) += 1);

    input.iter().map(|vec| vec.iter().skip(10).take(4).map(|s| if *lengths.get(&s.len()).unwrap() == 1 { 1 } else { 0 }).sum::<usize>()).sum()
}

fn puzzle_2(input: Input) -> usize {
    let mut lengths = HashMap::new();
    get_numbers().iter().for_each(|num| *lengths.entry(num.positions.len()).or_insert_with(|| 0) += 1);
    let numbers = get_numbers();

    println!("Printing stuff");
    println!("Input: {:?}", input);

    let mut result = 0;

    input.iter().for_each(|entry| {
        let mut possible_mappings = HashMap::new();
        possible_mappings.insert('a', HashSet::from([Position::Top, Position::TopLeft, Position::TopRight, Position::Middle, Position::BottomLeft, Position::BottomRight, Position::Bottom]));
        possible_mappings.insert('b', HashSet::from([Position::Top, Position::TopLeft, Position::TopRight, Position::Middle, Position::BottomLeft, Position::BottomRight, Position::Bottom]));
        possible_mappings.insert('c', HashSet::from([Position::Top, Position::TopLeft, Position::TopRight, Position::Middle, Position::BottomLeft, Position::BottomRight, Position::Bottom]));
        possible_mappings.insert('d', HashSet::from([Position::Top, Position::TopLeft, Position::TopRight, Position::Middle, Position::BottomLeft, Position::BottomRight, Position::Bottom]));
        possible_mappings.insert('e', HashSet::from([Position::Top, Position::TopLeft, Position::TopRight, Position::Middle, Position::BottomLeft, Position::BottomRight, Position::Bottom]));
        possible_mappings.insert('f', HashSet::from([Position::Top, Position::TopLeft, Position::TopRight, Position::Middle, Position::BottomLeft, Position::BottomRight, Position::Bottom]));
        possible_mappings.insert('g', HashSet::from([Position::Top, Position::TopLeft, Position::TopRight, Position::Middle, Position::BottomLeft, Position::BottomRight, Position::Bottom]));

        println!("  Entry: {:?}", entry);
        entry.iter().take(10).for_each(|word| {
            if *lengths.get(&word.len()).unwrap() == 1 {
                let number = numbers.iter().find(|n| n.positions.len() == word.len()).unwrap();
                let these_chars: Vec<char> = word.chars().collect();

                for (k, v) in possible_mappings.iter_mut() {
                    if these_chars.contains(k) {
                        for p in complement_position(&number.positions) {
                            v.remove(&p);
                        }
                    } else {
                        for p in number.positions.iter() {
                            v.remove(p);
                        }
                    }
                }
            }
        });

        for a in possible_mappings.get(&'a').unwrap() {
            for b in possible_mappings.get(&'b').unwrap() {
                for c in possible_mappings.get(&'c').unwrap() {
                    for d in possible_mappings.get(&'d').unwrap() {
                        for e in possible_mappings.get(&'e').unwrap() {
                            for f in possible_mappings.get(&'f').unwrap() {
                                for g in possible_mappings.get(&'g').unwrap() {
                                    let mut test_mapping = HashMap::new();
                                    test_mapping.insert('a', HashSet::from([a]));
                                    test_mapping.insert('b', HashSet::from([b]));
                                    test_mapping.insert('c', HashSet::from([c]));
                                    test_mapping.insert('d', HashSet::from([d]));
                                    test_mapping.insert('e', HashSet::from([e]));
                                    test_mapping.insert('f', HashSet::from([f]));
                                    test_mapping.insert('g', HashSet::from([g]));

                                    let numbers: Vec<Option<Number>> = entry.iter().take(10).map(|word| get_number_from(&test_mapping, word)).collect();

                                    if numbers.iter().fold(true, |acc, v| acc && v.is_some()) {
                                        let num_un: Vec<usize> = numbers.iter().map(|n| n.as_ref().unwrap().num).collect();
                                        let set: HashSet<usize> = HashSet::from_iter(num_un.iter().cloned());
                                        if set.len() == 10 {
                                            let print_friendly: Vec<String> = numbers
                                                .iter()
                                                .map(|o| match o {
                                                    None => "?".to_string(),
                                                    Some(num) => format!("{}", num.num),
                                                })
                                                .collect();

                                            let mut out_num = 0;
                                            let mut mult = 1000;
                                            entry.iter().skip(10).for_each(|word| {
                                                out_num += get_number_from(&test_mapping, word).unwrap().num * mult;
                                                mult /= 10;
                                            });
                                            println!("  Output number: {}", out_num);

                                            result += out_num;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    result
}

fn get_number_from(possible_mappings: &HashMap<char, HashSet<&Position>>, word: &str) -> Option<Number> {
    let mut positions_of_number = HashSet::new();

    for c in word.chars() {
        for map in possible_mappings.get(&c).unwrap() {
            positions_of_number.insert(map);
        }
    }

    for num in get_numbers() {
        let mut matches = true;
        for p in num.positions.iter() {
            if !positions_of_number.contains(&p) {
                matches = false;
            }
        }
        if num.positions.len() == positions_of_number.len() && matches {
            return Some(num);
        }
    }

    None
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum Position {
    Top,
    Middle,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn get_numbers() -> Vec<Number> {
    vec![
        Number { num: 8, positions: vec![Position::Top, Position::TopLeft, Position::TopRight, Position::Middle, Position::BottomLeft, Position::BottomRight, Position::Bottom] },
        Number { num: 0, positions: vec![Position::Top, Position::TopLeft, Position::TopRight, Position::BottomLeft, Position::BottomRight, Position::Bottom] },
        Number { num: 6, positions: vec![Position::Top, Position::TopLeft, Position::Middle, Position::BottomLeft, Position::BottomRight, Position::Bottom] },
        Number { num: 9, positions: vec![Position::Top, Position::TopLeft, Position::TopRight, Position::Middle, Position::BottomRight, Position::Bottom] },
        Number { num: 2, positions: vec![Position::Top, Position::TopRight, Position::Middle, Position::BottomLeft, Position::Bottom] },
        Number { num: 3, positions: vec![Position::Top, Position::TopRight, Position::Middle, Position::BottomRight, Position::Bottom] },
        Number { num: 5, positions: vec![Position::Top, Position::TopLeft, Position::Middle, Position::BottomRight, Position::Bottom] },
        Number { num: 4, positions: vec![Position::TopLeft, Position::TopRight, Position::Middle, Position::BottomRight] },
        Number { num: 7, positions: vec![Position::Top, Position::TopRight, Position::BottomRight] },
        Number { num: 1, positions: vec![Position::TopRight, Position::BottomRight] },
    ]
}

fn complement_position(positions: &Vec<Position>) -> Vec<Position> {
    let mut complement = vec![Position::Top, Position::TopLeft, Position::TopRight, Position::Middle, Position::BottomLeft, Position::BottomRight, Position::Bottom];
    complement.iter().filter(|p| !positions.contains(p)).map(|p| p.clone()).collect()
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Number {
    num: usize,
    positions: Vec<Position>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(26, puzzle_1(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(479, puzzle_1(parse(include_str!("puzzle_1.txt"))));

        assert_eq!(61229, puzzle_2(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(1041746, puzzle_2(parse(include_str!("puzzle_1.txt"))));
    }

    fn parse(s: &str) -> Input {
        s.lines().map(|line| line.replace(" |", "").split(" ").map(|s| s.to_string()).collect()).collect()
    }
}
