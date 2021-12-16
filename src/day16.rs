use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::str::Chars;

fn read_as_binary_string(filename: &str) -> Result<String> {
    let mut f = File::open(filename)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    let list: Vec<String> = buf
        .trim_end()
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect();
    Ok(list.iter().flat_map(|s| s.chars()).collect())
}

#[derive(Debug)]
struct Node {
    version: usize,
    inner: Inner,
}

#[derive(Debug)]
enum Inner {
    Literal(usize),
    Operation(usize, Vec<Node>),
}

impl Node {
    fn from_chars(chars: &mut Chars) -> Result<(Self, usize)> {
        let version_str: String = chars.take(3).collect();
        let version = usize::from_str_radix(&version_str, 2)?;
        let opcode_str: String = chars.take(3).collect();
        let opcode = usize::from_str_radix(&opcode_str, 2)?;

        let mut bits_read = 6;

        let inner = if opcode == 4 {
            let mut num = String::new();
            loop {
                let last_flag = chars.next().unwrap();
                let s: String = chars.take(4).collect();
                num += &s;
                bits_read += 5;
                if last_flag == '0' {
                    break;
                }
            }
            let number = usize::from_str_radix(&num, 2)?;
            Inner::Literal(number)
        } else {
            let length_based = chars.next().unwrap() == '0';

            if length_based {
                let length: String = chars.take(15).collect();
                let length = usize::from_str_radix(&length, 2)?;
                bits_read += 16;
                let mut sub_nodes = Vec::new();
                let mut new_bits_read = 0;
                while new_bits_read < length {
                    let (node, sub_length) = Node::from_chars(chars)?;
                    sub_nodes.push(node);
                    new_bits_read += sub_length;
                }
                bits_read += new_bits_read;
                Inner::Operation(opcode, sub_nodes)
            } else {
                let count: String = chars.take(11).collect();
                let count = usize::from_str_radix(&count, 2)?;
                bits_read += 12;
                let mut sub_nodes = Vec::new();
                for _ in 0..count {
                    let (node, sub_length) = Node::from_chars(chars)?;
                    sub_nodes.push(node);
                    bits_read += sub_length;
                }
                Inner::Operation(opcode, sub_nodes)
            }
        };
        Ok((Node { version, inner }, bits_read))
    }

    fn sum_versions(&self) -> usize {
        let sub_sum = match &self.inner {
            Inner::Literal(_) => 0,
            Inner::Operation(_, sub_nodes) => sub_nodes.iter().map(Node::sum_versions).sum(),
        };
        self.version + sub_sum
    }

    fn evaluate(&self) -> usize {
        match &self.inner {
            Inner::Literal(x) => *x,
            Inner::Operation(opcode, sub_nodes) => match opcode {
                0 => sub_nodes.iter().map(Node::evaluate).sum(),
                1 => sub_nodes.iter().map(Node::evaluate).product(),
                2 => sub_nodes.iter().map(Node::evaluate).min().unwrap(),
                3 => sub_nodes.iter().map(Node::evaluate).max().unwrap(),
                5 => {
                    if sub_nodes[0].evaluate() > sub_nodes[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if sub_nodes[0].evaluate() < sub_nodes[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if sub_nodes[0].evaluate() == sub_nodes[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("Unrecognized OpCode"),
            },
        }
    }
}

fn q1(filename: &str) -> Result<usize> {
    let to_parse = read_as_binary_string(filename)?;
    let mut chars = to_parse.chars();
    let (node, _) = Node::from_chars(&mut chars)?;
    Ok(node.sum_versions())
}

fn q2(filename: &str) -> Result<usize> {
    let to_parse = read_as_binary_string(filename)?;
    let mut chars = to_parse.chars();
    let (node, _) = Node::from_chars(&mut chars)?;
    Ok(node.evaluate())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(q1("./data/day16.txt").unwrap(), 960);
        assert_eq!(q2("./data/day16.txt").unwrap(), 12301926782560);
    }
}
