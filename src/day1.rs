use crate::util::parse_numbers;
use anyhow::Result;

#[allow(dead_code)]
fn q1(filename: &str) -> Result<usize> {
    let numbers = parse_numbers(filename)?;
    assert!(numbers.len() > 1);
    Ok(num_increasing(numbers))
}

#[allow(dead_code)]
fn q2(filename: &str) -> Result<usize> {
    let numbers = parse_numbers(filename)?;
    assert!(numbers.len() > 2);
    let mut window = Vec::new();
    for i in 0..numbers.len() - 2 {
        window.push(numbers[i] + numbers[i + 1] + numbers[i + 2]);
    }
    Ok(num_increasing(window))
}

#[allow(dead_code)]
fn num_increasing(numbers: Vec<usize>) -> usize {
    let mut count = 0;
    let mut prev = numbers[0];
    for number in numbers.iter().skip(1) {
        if *number > prev {
            count += 1;
        }
        prev = *number;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(q1("./data/day1.txt").unwrap(), 1548);
        assert_eq!(q2("./data/day1.txt").unwrap(), 1589);
    }
}
