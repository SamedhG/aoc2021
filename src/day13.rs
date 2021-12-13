use std::collections::HashSet;

use crate::util::parse_strings;
use anyhow::Result;

type Coord = (usize, usize);

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    fn apply(&self, coords: HashSet<Coord>) -> HashSet<Coord> {
        match self {
            Fold::X(loc) => coords
                .into_iter()
                .map(|(x, y)| if x < *loc { (x, y) } else { (2 * loc - x, y) })
                .collect(),
            Fold::Y(loc) => coords
                .into_iter()
                .map(|(x, y)| if y < *loc { (x, y) } else { (x, 2 * loc - y) })
                .collect(),
        }
    }
}

fn parse(filename: &str) -> Result<(HashSet<Coord>, Vec<Fold>)> {
    let strings = parse_strings(filename)?;
    let mut coords = HashSet::new();
    let mut lines = strings.iter();

    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let mut split = line.split(',');
        let x = split.next().unwrap().parse::<usize>()?;
        let y = split.next().unwrap().parse::<usize>()?;
        coords.insert((x, y));
    }
    let mut folds = Vec::new();
    while let Some(line) = lines.next() {
        let mut split = line.split('=');
        let direction = split.next().unwrap();
        let direction = direction.chars().last().unwrap();
        let location = split.next().unwrap().parse::<usize>()?;
        let fold = match direction {
            'x' => Fold::X(location),
            'y' => Fold::Y(location),
            _ => panic!("unreachable direction"),
        };
        folds.push(fold);
    }

    Ok((coords, folds))
}

fn q1(filename: &str) -> Result<usize> {
    let (coords, folds) = parse(filename)?;
    let new_coords = folds[0].apply(coords);
    Ok(new_coords.len())
}

fn display(points: &HashSet<Coord>) {
    let width = points
        .iter()
        .max_by(|first, second| first.0.cmp(&second.0))
        .unwrap()
        .0
        + 1;
    let height = points
        .iter()
        .max_by(|first, second| first.1.cmp(&second.1))
        .unwrap()
        .1
        + 1;
    let mut image = vec![vec![false; width]; height];
    for (x, y) in points.iter() {
        image[*y][*x] = true;
    }

    let mut s = String::new();
    for line in image {
        for on in line {
            if on {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    println!("{}", s);
}

fn q2(filename: &str) -> Result<usize> {
    let (mut coords, folds) = parse(filename)?;
    for fold in folds {
        coords = fold.apply(coords);
    }
    display(&coords);
    // Prints the code JGAJEFKU
    Ok(coords.len())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        assert_eq!(q1("./data/day13.txt").unwrap(), 592);
        assert_eq!(q2("./data/day13.txt").unwrap(), 94);
    }
}
