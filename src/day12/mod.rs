use std::collections::{HashMap, HashSet};

pub fn main() {
    println!("Day");
}

type Input = HashMap<String, Vec<String>>;

fn puzzle_1(input: Input) -> usize {
    dfs(&input, "start".to_string(), &mut HashMap::new(), 0)
}

fn puzzle_2(input: Input) -> usize {
    dfs(&input, "start".to_string(), &mut HashMap::new(), 1)
}

fn dfs(input: &Input, pos: String, visited: &mut HashMap<String, usize>, can_revisit_small_case: usize) -> usize {
    if pos.eq("end") {
        return 1;
    }

    let small_cave = pos.chars().all(char::is_lowercase);
    if small_cave {
        *visited.entry(pos.clone()).or_insert_with(|| 0) += 1;
    }

    let mut hits = 0;
    for next in input.get(&pos).unwrap() {
        let visits = *visited.entry(next.clone()).or_insert_with(|| 0);

        if visits == 0 {
            hits += dfs(&input, next.clone(), visited, can_revisit_small_case);
        } else if can_revisit_small_case > 0 {
            hits += dfs(&input, next.clone(), visited, can_revisit_small_case - 1);
        }
    }

    if small_cave {
        *visited.entry(pos.clone()).or_insert_with(|| 0) -= 1;
    }

    hits
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(10, puzzle_1(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(3563, puzzle_1(parse(include_str!("puzzle_1.txt"))));

        assert_eq!(36, puzzle_2(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(1, puzzle_2(parse(include_str!("puzzle_1.txt"))));
    }

    fn parse(s: &str) -> Input {
        let mut map = HashMap::new();
        s.lines().for_each(|line| {
            let spl: Vec<&str> = line.split("-").collect();
            if let [a, b] = spl[..] {
                if !b.eq("start") {
                    map.entry(a.to_string()).or_insert_with(|| vec![]).push(b.to_string());
                }
                if !a.eq("start") {
                    map.entry(b.to_string()).or_insert_with(|| vec![]).push(a.to_string());
                }
            }
        });

        map
    }
}
