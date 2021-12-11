use crate::util::parse_strings;
use anyhow::Result;

fn q1(filename: &str) -> Result<usize> {
    let codes = parse_strings(filename)?;
    let mut score = 0;
    for code in codes {
        let mut stack = Vec::new();
        for c in code.chars() {
            match c {
                '(' | '{' | '[' | '<' => stack.push(c),
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        score += 3;
                        break;
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        score += 57;
                        break;
                    }
                }
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        score += 1197;
                        break;
                    }
                }
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        score += 25137;
                        break;
                    }
                }
                _ => panic!("unexpected character"),
            }
        }
    }
    Ok(score)
}


fn q2(filename: &str) -> Result<usize> {
    let codes = parse_strings(filename)?;
    let mut scores = Vec::new();
    for code in codes {
        let mut stack = Vec::new();
        let mut corrupted = false;
        for c in code.chars() {
            match c {
                '(' | '{' | '[' | '<' => stack.push(c),
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        corrupted = true;
                        break;
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        corrupted = true;
                        break;
                    }
                }
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        corrupted = true;
                        break;
                    }
                }
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        corrupted = true;
                        break;
                    }
                }
                _ => panic!("unexpected character"),
            }
        }
        if !corrupted {
            let mut score = 0;
            for brace in stack.iter().rev() {
                score *= 5;
                match brace {
                    '(' => score += 1,
                    '[' => score += 2,
                    '{' => score += 3,
                    '<' => score += 4,
                    _ => panic!("unexpected"),
                }
            }
            scores.push(score);
        }
    }
    scores.sort();
    //dbg!(&scores);
    Ok(scores[scores.len()/2])
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(q1("./data/day10.txt").unwrap(), 319329);
        assert_eq!(q2("./data/day10.txt").unwrap(), 1589);
    }
}
