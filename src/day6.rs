use crate::util::parse_numbers_comma;
use anyhow::Result;

fn num_lanternfish(filename: &str, days: usize) -> Result<usize> {
    let numbers = parse_numbers_comma(filename)?;
    let mut counts = [0; 9];
    for n in numbers {
        counts[n] += 1;
    }

    for _day in 0..days {
        let zero_count = counts[0];
        for i in 0..8 {
            counts[i] = counts[i+1]
        }
        counts[8] = zero_count;
        counts[6] += zero_count;
    }
    
    Ok(counts.iter().sum())
}

fn q1(filename: &str) -> Result<usize> {
    num_lanternfish(filename, 80)
}

fn q2(filename: &str) -> Result<usize> {
    num_lanternfish(filename, 256)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(q1("./data/day6.txt").unwrap(), 395627);
        assert_eq!(q2("./data/day6.txt").unwrap(), 1767323539209);
    }
}
