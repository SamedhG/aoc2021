use crate::util::parse_strings;
use anyhow::Result;

struct Bingo([[(usize, bool); 5]; 5]);

impl Bingo {
    // Use the first 5 lines of this iterator to generate a bingo board
    fn parse(iter: &mut dyn Iterator<Item = &String>) -> Self {
        let mut board = [[(0, false); 5]; 5];
        for (row_idx, row) in iter.take(5).enumerate() {
            for (col_idx, val) in row
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .enumerate()
            {
                board[row_idx][col_idx] = (val, false);
            }
        }
        Bingo(board)
    }

    fn mark(&mut self, value: usize) {
        for row in &mut self.0[..] {
            for (num, seen) in &mut row[..] {
                if *num == value {
                    *seen = true;
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        for i in 0..5 {
            let mut row = true;
            let mut col = true;
            for j in 0..5 {
                row &= self.0[i][j].1;
                col &= self.0[j][i].1;
            }
            if row || col {
                return true;
            }
        }
        false
    }

    fn sum_unmarked(&self) -> usize {
        let mut sum = 0;
        for row in self.0 {
            for (num, marked) in row {
                if !marked {
                    sum += num;
                }
            }
        }
        sum
    }
}

/// Parse out the call list and the boards available
fn parse(filename: &str) -> Result<(Vec<usize>, Vec<Bingo>)> {
    let data = parse_strings(filename)?;
    let mut iter = data.iter();
    let call_line = iter.next().unwrap();
    let numbers = call_line
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut boards = Vec::new();
    while iter.next().is_some() {
        boards.push(Bingo::parse(&mut iter));
    }
    Ok((numbers, boards))
}

/// Find the winning board
fn q1(filename: &str) -> Result<usize> {
    let (numbers, mut boards) = parse(filename)?;
    for number in numbers.into_iter() {
        boards.iter_mut().for_each(|b| b.mark(number));
        for board in &boards {
            if board.has_won() {
                return Ok(number * board.sum_unmarked());
            }
        }
    }

    Ok(1)
}

/// Find the losing board
fn q2(filename: &str) -> Result<usize> {
    let (numbers, mut boards) = parse(filename)?;
    for number in numbers.into_iter() {
        boards.iter_mut().for_each(|b| b.mark(number));
        if boards.len() == 1 {
            if boards[0].has_won() {
                return Ok(boards[0].sum_unmarked() * number);
            }
        } else {
            boards = boards.into_iter().filter(|x| !x.has_won()).collect();
        }
    }

    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(q1("./data/day04.txt").unwrap(), 49860);
        assert_eq!(q2("./data/day04.txt").unwrap(), 24628);
    }
}
