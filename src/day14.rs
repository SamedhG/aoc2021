use crate::util::parse_strings;
use anyhow::Result;

use std::collections::HashMap;

type Rules = HashMap<(char, char), char>;
type Counts = HashMap<char, usize>;

fn apply_rules(rules: &Rules, input: &[char]) -> Vec<char> {
    let mut output = Vec::new();
    for i in 0..input.len() - 1 {
        output.push(input[i]);
        if let Some(c) = rules.get(&(input[i], input[i + 1])) {
            output.push(*c);
        }
    }
    output.push(input[input.len() - 1]);
    output
}

fn count(list: &[char]) -> Counts {
    let mut counts = HashMap::new();
    for c in list {
        add(&mut counts, *c, 1);
    }
    counts
}

fn parse(filename: &str) -> Result<(Vec<char>, Rules)> {
    let lines = parse_strings(filename)?;
    let mut rules = HashMap::new();
    let mut iter = lines.iter();
    let code = iter.next().unwrap().chars().collect();
    for line in iter.skip(1) {
        let mut split = line.split(" -> ");
        let mut input = split.next().unwrap().chars();
        let i0 = input.next().unwrap();
        let i1 = input.next().unwrap();
        let output = split.next().unwrap().chars().next().unwrap();
        rules.insert((i0, i1), output);
    }
    Ok((code, rules))
}

fn q1(filename: &str) -> Result<usize> {
    let (mut input, rules) = parse(filename)?;
    for _ in 0..10 {
        input = apply_rules(&rules, &input);
    }
    let counts = count(&input);
    let max = counts.iter().map(|x| *x.1).max().unwrap();
    let min = counts.iter().map(|x| *x.1).min().unwrap();
    Ok(max - min)
}

fn add(counts: &mut Counts, c: char, value: usize) {
    let to_insert = counts.get(&c).unwrap_or(&0) + value;
    counts.insert(c, to_insert);
}

fn add_all(counts: &mut Counts, from: &Counts) {
    for (k, v) in from {
        add(counts, *k, *v);
    }
}

// Misses the last character in the counts
fn count_num_output(
    rules: &Rules,
    cached_counts: &mut HashMap<(char, char, usize), Counts>,
    c0: char,
    c1: char,
    steps: usize,
    total_counts: &mut Counts,
) {
    // Base case
    if steps == 0 {
        add(total_counts, c0, 1);
        return;
    }
    // cache case
    if let Some(counts) = cached_counts.get(&(c0, c1, steps)) {
        add_all(total_counts, counts);
        return;
    }
    // Normal Case
    let mut new_counts = HashMap::new();
    if let Some(x) = rules.get(&(c0, c1)) {
        count_num_output(&rules, cached_counts, c0, *x, steps - 1, &mut new_counts);
        count_num_output(&rules, cached_counts, *x, c1, steps - 1, &mut new_counts);
    } else {
        new_counts.insert(c0, 1);
    }
    add_all(total_counts, &new_counts);
    cached_counts.insert((c0, c1, steps), new_counts);
}

fn q2(filename: &str) -> Result<usize> {
    let (input, rules) = parse(filename)?;

    let mut cache = HashMap::new();
    let mut counts = HashMap::new();
    counts.insert(input[input.len() - 1], 1);

    for i in 0..input.len() - 1 {
        count_num_output(&rules, &mut cache, input[i], input[i + 1], 40, &mut counts);
    }

    let max = counts.iter().map(|x| *x.1).max().unwrap();
    let min = counts.iter().map(|x| *x.1).min().unwrap();
    Ok(max - min)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(q1("./data/day14.txt").unwrap(), 2740);
        assert_eq!(q2("./data/day14.txt").unwrap(), 2959788056211);
    }
}
