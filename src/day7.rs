use crate::util::parse_numbers_comma;
use anyhow::Result;

fn q1(filename: &str) -> Result<isize> {
    let mut numbers = parse_numbers_comma(filename)?;
    numbers.sort();
    let median = numbers[numbers.len() / 2];
    let result = numbers.iter().map(|x| (*x - median).abs()).sum();
    Ok(result)
}

fn q2(filename: &str) -> Result<isize> {
    let distance = |from: isize, to: isize| {
        let n = (from - to).abs();
        (n * (n + 1)) / 2
    };
    let numbers = parse_numbers_comma(filename)?;
    let max = *(numbers.iter().max().unwrap()) + 1;
    let min = *(numbers.iter().min().unwrap());
    let value = (min..max)
        .map(|target| numbers.iter().map(|x| distance(*x, target)).sum::<isize>())
        .min();

    Ok(value.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(q1("./data/day7.txt").unwrap(), 326132);
        assert_eq!(q2("./data/day7.txt").unwrap(), 2);
    }
}
