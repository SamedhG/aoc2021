use crate::util::parse_numbers;
use anyhow::Result;

fn q1(filename: &str) -> Result<u32> {
    let base = 2;
    let mut numbers = parse_numbers(filename, base)?;
    let target = numbers.len() / 2;
    let mut gamma = 0;
    let mut epsilon = 0;
    for digit in 0..12 {
        let mut num_ones = 0;
        for number in &numbers {
            if number % 2 == 1 {
                num_ones += 1;
            }
        }
        if num_ones > target {
            gamma += base.pow(digit);
        } else {
            epsilon += base.pow(digit);
        }
        numbers = numbers.iter().map(|n| n / 2).collect();
    }
    Ok(gamma * epsilon)
}

fn q2(filename: &str) -> Result<usize> {
    let base = 2;
    let numbers = parse_numbers(filename, base)?;
    let o2 = find_report(numbers.clone(), true);
    let co2 = find_report(numbers, false);
    Ok(o2 * co2)
}

fn find_report(mut numbers: Vec<usize>, o2: bool) -> usize {
    let mut digit = 11;
    while numbers.len() >= 2 {
        let mut num_ones = 0;
        for number in &numbers {
            num_ones += nth_digit(*number, digit);
        }
        // This can probably be cleaned up a lot
        let digit_to_keep = if o2 && numbers.len() % 2 == 0 && num_ones == numbers.len() / 2 {
            // they are equal and we are o2
            1
        } else if numbers.len() % 2 == 0 && num_ones == numbers.len() / 2 {
            // They are equal and we are c02
            0
        } else if o2 && num_ones >= numbers.len() / 2 {
            // 1 is more common and we are o2
            1
        } else if num_ones >= numbers.len() / 2 {
            // 1 is more common and we are c02
            0
        } else if o2 {
            // 0 is more common and we are o2
            0
        } else {
            // 0 is more common and we are c02
            1
        };
        numbers = numbers
            .iter()
            .filter(|num| nth_digit(**num, digit) == digit_to_keep)
            .cloned()
            .collect();
        digit -= 1;
    }
    return numbers[0];
}

fn nth_digit(number: usize, digit: u32) -> usize {
    let two: u32 = 2;
    (number as u32 / two.pow(digit) % 2) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(q1("./data/day03.txt").unwrap(), 3429254);
        assert_eq!(q2("./data/day03.txt").unwrap(), 5410338);
    }
}
