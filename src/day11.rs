use crate::util::parse_strings;
use anyhow::Result;
use std::collections::HashSet;

type Map = [[usize; 10]; 10];
type Coord = (usize, usize);

fn parse(filename: &str) -> Result<Map> {
    let lines = parse_strings(filename)?;
    let mut result = [[0; 10]; 10];
    for (i, line) in lines.into_iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            result[i][j] = ch.to_digit(10).unwrap() as usize;
        }
    }
    Ok(result)
}

fn neighbors(coord: Coord) -> Vec<Coord> {
    let row_start = if coord.0 == 0 { 0 } else { coord.0 - 1 };
    let row_end = (coord.0 + 1).min(9);
    let col_start = if coord.1 == 0 { 0 } else { coord.1 - 1 };
    let col_end = (coord.1 + 1).min(9);
    let mut results = Vec::new();
    for row in row_start..row_end + 1 {
        for col in col_start..col_end + 1 {
            if !(row == coord.0 && col == coord.1) {
                results.push((row, col));
            }
        }
    }
    results
}

// returns the number of flashes that occured during this step
fn forward_step(map: &mut Map) -> usize {
    let mut over_nines = HashSet::new();
    for i in 0..10 {
        for j in 0..10 {
            map[i][j] += 1;
            if map[i][j] > 9 {
                over_nines.insert((i, j));
            }
        }
    }
    let mut next_flash: HashSet<Coord> = over_nines.clone();
    while next_flash.len() > 0 {
        let current_flash = next_flash;
        next_flash = HashSet::new();
        for coord in current_flash {
            for (i, j) in neighbors(coord) {
                map[i][j] += 1;
                if map[i][j] > 9 && !over_nines.contains(&(i, j)) {
                    next_flash.insert((i, j));
                    over_nines.insert((i, j));
                }
            }
        }
    }
    for (i, j) in &over_nines {
        map[*i][*j] = 0;
    }
    over_nines.len()
}

fn all_flashing(map: &Map) -> bool {
    let mut all_flashing = true;
    for i in 0..10 {
        for j in 0..10 {
            all_flashing &= map[i][j] == 0
        }
    }
    all_flashing
}

fn q1(filename: &str) -> Result<usize> {
    let mut map = parse(filename)?;
    let mut total_flashes = 0;
    for _ in 0..100 {
        total_flashes += forward_step(&mut map);
    }
    Ok(total_flashes)
}

fn q2(filename: &str) -> Result<usize> {
    let mut map = parse(filename)?;
    let mut step = 0;
    while !all_flashing(&map) {
        step += 1;
        forward_step(&mut map);
    }
    Ok(step)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(q1("./data/day11.txt").unwrap(), 1729);
        assert_eq!(q2("./data/day11.txt").unwrap(), 237);
    }
}
