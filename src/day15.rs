use crate::util::parse_strings;
use anyhow::Result;
use std::collections::VecDeque;

type Map = Vec<Vec<usize>>;
type Coord = (usize, usize);

fn parse(filename: &str) -> Result<Map> {
    let lines = parse_strings(filename)?;
    let mut result = vec![vec![0; lines[0].len()]; lines.len()];
    for (i, line) in lines.into_iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            result[i][j] = ch.to_digit(10).unwrap() as usize;
        }
    }
    Ok(result)
}

fn neighbors(coord: Coord, map: &Map) -> Vec<Coord> {
    let mut results = Vec::new();
    if coord.0 > 0 {
        results.push((coord.0 - 1, coord.1));
    }
    if coord.1 > 0 {
        results.push((coord.0, coord.1 - 1))
    }
    if coord.0 < map.len() - 1 {
        results.push((coord.0 + 1, coord.1));
    }
    if coord.1 < map[0].len() - 1 {
        results.push((coord.0, coord.1 + 1));
    }
    results
}

fn cheapest_path(from: Coord, to: Coord, map: &Map) -> f32 {
    let mut risks = vec![vec![f32::INFINITY; map[0].len()]; map.len()];
    risks[to.0][to.1] = map[to.0][to.1] as f32;
    let mut queue = VecDeque::from([to]);
    while let Some(new_from) = queue.pop_front() {
        let my_risk = risks[new_from.0][new_from.1];
        let neighbors = neighbors(new_from, map);
        for n in neighbors {
            let new_risk = my_risk + map[n.0][n.1] as f32;
            if risks[n.0][n.1] > new_risk {
                risks[n.0][n.1] = new_risk;
                queue.push_back(n);
            }
        }
    }
    risks[from.0][from.1] - map[from.0][from.1] as f32
}

fn quintuple_map(map: &Map) -> Map {
    let nrows = map.len();
    let ncols = map[0].len();
    map.iter()
        .map(|row| {
            row.iter()
                .cycle()
                .take(nrows * 5)
                .enumerate()
                .map(|(i, x)| {
                    let new_risk = x + (i / nrows);
                    if new_risk > 9 {
                        new_risk - 9
                    } else {
                        new_risk
                    }
                })
        })
        .cycle()
        .take(ncols * 5)
        .enumerate()
        .map(|(i, row)| {
            row.map(|x| {
                let new_risk = x + (i / nrows);
                if new_risk > 9 {
                    new_risk - 9
                } else {
                    new_risk
                }
            })
            .collect()
        })
        .collect()
}

fn q1(filename: &str) -> Result<usize> {
    let map = parse(filename)?;
    let from = (0, 0);
    let to = (map.len() - 1, map[0].len() - 1);
    Ok(cheapest_path(from, to, &map) as usize)
}

fn q2(filename: &str) -> Result<usize> {
    let map = parse(filename)?;
    let map = quintuple_map(&map);
    let from = (0, 0);
    let to = (map.len() - 1, map[0].len() - 1);
    Ok(cheapest_path(from, to, &map) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(q1("./data/day15.txt").unwrap(), 441);
        assert_eq!(q2("./data/day15.txt").unwrap(), 2849);
    }
}
