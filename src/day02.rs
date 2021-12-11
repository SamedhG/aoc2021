use crate::util::parse_strings;
use anyhow::Result;

#[derive(Debug)]
enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

fn parse_commands(filename: &str) -> Result<Vec<Command>> {
    let commands = parse_strings(filename)?;
    let converted_commands: Vec<Command> = commands
        .iter()
        .map(|c| {
            let mut split = c.split_whitespace();
            match split.next().unwrap() {
                "forward" => Command::Forward(split.next().unwrap().parse().unwrap()),
                "up" => Command::Up(split.next().unwrap().parse().unwrap()),
                "down" => Command::Down(split.next().unwrap().parse().unwrap()),
                _ => panic!("unknown command"),
            }
        })
        .collect();
    Ok(converted_commands)
}

// Returns depth * horizontal position
fn q1(filename: &str) -> Result<usize> {
    let mut position = 0;
    let mut depth = 0;
    let commands = parse_commands(filename)?;
    for c in commands {
        match c {
            Command::Forward(x) => position += x,
            Command::Up(x) => depth -= x,
            Command::Down(x) => depth += x,
        }
    }
    Ok(position * depth)
}

// Returns depth * horizontal position
fn q2(filename: &str) -> Result<usize> {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;
    let commands = parse_commands(filename)?;
    for c in commands {
        match c {
            Command::Forward(x) => {
                position += x;
                depth += aim * x;
            }
            Command::Up(x) => aim -= x,
            Command::Down(x) => aim += x,
        }
    }
    Ok(position * depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(q1("./data/day02.txt").unwrap(), 1924923);
        assert_eq!(q2("./data/day02.txt").unwrap(), 1982495697);
    }
}
