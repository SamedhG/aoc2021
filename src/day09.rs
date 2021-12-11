use std::collections::HashSet;

use crate::util::parse_strings;
use anyhow::Result;

type HeightMap = Vec<Vec<u32>>;

type Coord = (usize, usize);

fn parse(filename: &str) -> Result<HeightMap> {
    let strs = parse_strings(filename)?;
    Ok(strs
        .into_iter()
        .map(|s| s.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect())
}

fn find_lowest_points(height_map: &HeightMap) -> Vec<Coord> {
    let mut coords = Vec::new();
    let num_rows = height_map.len();
    let num_cols = height_map[0].len();
    for row in 0..num_rows {
        for col in 0..num_cols {
            let val = height_map[row][col];
            let is_lowest = (row == 0 || val < height_map[row - 1][col])
                && (row == num_rows - 1 || val < height_map[row + 1][col])
                && (col == 0 || val < height_map[row][col - 1])
                && (col == num_cols - 1 || val < height_map[row][col + 1]);
            if is_lowest {
                coords.push((row, col));
            }
        }
    }
    coords
}

fn q1(filename: &str) -> Result<u32> {
    let height_map = parse(filename)?;
    let lowest_points = find_lowest_points(&height_map);
    let total_risk = lowest_points
        .iter()
        .map(|(r, c)| height_map[*r][*c] + 1)
        .sum();
    Ok(total_risk)
}

fn non_nine_neighbors(hm: &HeightMap, coord: Coord) -> Vec<Coord> {
    let num_rows = hm.len();
    let num_cols = hm[0].len();
    let mut results = Vec::new();

    if coord.0 != 0 && hm[coord.0 - 1][coord.1] != 9 {
        results.push((coord.0 - 1, coord.1));
    }
    if coord.0 != num_rows - 1 && hm[coord.0 + 1][coord.1] != 9 {
        results.push((coord.0 + 1, coord.1));
    }
    if coord.1 != 0 && hm[coord.0][coord.1 - 1] != 9 {
        results.push((coord.0, coord.1 - 1));
    }
    if coord.1 != num_cols - 1 && hm[coord.0][coord.1 + 1] != 9 {
        results.push((coord.0, coord.1 + 1));
    }
    results
}

fn basin_size(hm: &HeightMap, coord: Coord) -> usize {
    let mut basin = HashSet::new();
    let mut to_check = vec![coord];
    while to_check.len() > 0 {
        let check = to_check.pop().unwrap();
        let nnn = non_nine_neighbors(hm, check);
        for coord in nnn {
            if !basin.contains(&coord) {
                basin.insert(coord);
                to_check.push(coord);
            }
        }
    }

    basin.len()
}

fn q2(filename: &str) -> Result<usize> {
    let height_map = parse(filename)?;
    let lowest_points = find_lowest_points(&height_map);
    let mut result: Vec<usize> = lowest_points
        .iter()
        .map(|x| basin_size(&height_map, *x))
        .collect();
    result.sort();
    let product = result.into_iter().rev().take(3).product();
    Ok(product)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        assert_eq!(q1("./data/day09.txt").unwrap(), 512);
        assert_eq!(q2("./data/day09.txt").unwrap(), 1600104);
    }
}
