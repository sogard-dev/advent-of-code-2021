use std::fmt::Display;

pub fn main() {
    println!("Day");
}

type Input = Vec<Vec<Token>>;


fn puzzle_1(input: Input) -> usize {
    let mut points = 0;
    for line in input {
        let mut stack = vec![];

        for token in line {
            let should_add = match token {
                Token::StartA => true,
                Token::StartB => true,
                Token::StartC => true,
                Token::StartD => true,
                _ => false
            };

            if should_add {
                stack.push(token);
            } else {
                let popped = stack.pop().unwrap();
                let expect = match popped {
                    Token::StartA => Token::EndA,
                    Token::StartB => Token::EndB,
                    Token::StartC => Token::EndC,
                    Token::StartD => Token::EndD,
                    _ => panic!("Ohh noes")
                };

                if !expect.eq(&token) {
                    println!("Expected {}, was {}", expect, token);
                    points += token.points();
                }
            }
        }
    }

    points
}

fn puzzle_2(input: Input) -> usize {
    let mut points = vec![];

    for line in input {
        let mut stack = vec![];

        for token in line {
            let should_add = match token {
                Token::StartA => true,
                Token::StartB => true,
                Token::StartC => true,
                Token::StartD => true,
                _ => false
            };

            if should_add {
                stack.push(token);
            } else {
                let popped = stack.pop().unwrap();
                let expect = match popped {
                    Token::StartA => Token::EndA,
                    Token::StartB => Token::EndB,
                    Token::StartC => Token::EndC,
                    Token::StartD => Token::EndD,
                    _ => panic!("Ohh noes")
                };

                if !expect.eq(&token) {
                    stack.clear();
                    break;
                }
            }
        }

        if stack.len() > 0 {
            let mut extra = vec![];
            while let Some(popped) = stack.pop() {
                let expect = match popped {
                    Token::StartA => Token::EndA,
                    Token::StartB => Token::EndB,
                    Token::StartC => Token::EndC,
                    Token::StartD => Token::EndD,
                    _ => panic!("Ohh noes")
                };

                extra.push(expect);
            }

            let mut score = 0;
            for token in extra {
                score = score * 5;
                score += match token {
                    Token::EndA => 1,
                    Token::EndB => 2,
                    Token::EndC => 3,
                    Token::EndD => 4,
                    _ => panic!("Shouldnt")
                };
            }

            points.push(score);
        }
    }

    println!("Scores: {:?}", points);
    points.sort();

    points[points.len()/2]
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum Token {
    StartA, EndA,
    StartB, EndB,
    StartC, EndC,
    StartD, EndD
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Token::StartA => "(",
            Token::EndA => ")",
            Token::StartB => "[",
            Token::EndB => "]",
            Token::StartC => "{",
            Token::EndC => "}",
            Token::StartD => "<",
            Token::EndD => ">",
        })
    }
}

impl Token {
    fn points(&self) -> usize {
        match self {
            Token::EndA => 3,
            Token::EndB => 57,
            Token::EndC => 1197,
            Token::EndD => 25137,
            _ => panic!("No points for token {:?}", self)
        }
    }

    fn from_char(c: char) -> Token {
        match c {
            '(' => Token::StartA,
            ')' => Token::EndA,
            '[' => Token::StartB,
            ']' => Token::EndB,
            '{' => Token::StartC,
            '}' => Token::EndC,
            '<' => Token::StartD,
            '>' => Token::EndD,
            _ => panic!("Doh")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(26397, puzzle_1(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(319233, puzzle_1(parse(include_str!("puzzle_1.txt"))));

        assert_eq!(288957, puzzle_2(parse(include_str!("puzzle_1_test.txt"))));
        assert_eq!(1118976874, puzzle_2(parse(include_str!("puzzle_1.txt"))));
    }

    fn parse(s: &str) -> Input {
        s.lines().map(|line| line.chars().map(|c| Token::from_char(c)).collect()).collect()
    }
}
