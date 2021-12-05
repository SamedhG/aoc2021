use crate::util::parse_strings;
use anyhow::Result;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point(usize, usize);

impl Point {
    fn from_str(s: &str) -> Self {
        let mut split = s.split(',');
        let x = split.next().unwrap().parse::<usize>().unwrap();
        let y = split.next().unwrap().parse::<usize>().unwrap();
        Self(x, y)
    }
}

fn gen_range_incl(start: usize, end: usize) -> Box<dyn Iterator<Item = usize>> {
    if start <= end {
        return Box::new(start..end + 1);
    } else {
        return Box::new((end..start + 1).rev());
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn from_str(s: &str) -> Self {
        let mut split = s.split(" -> ");
        let from = Point::from_str(split.next().unwrap());
        let to = Point::from_str(split.next().unwrap());
        Self { from, to }
    }

    fn is_straight(&self) -> bool {
        self.from.0 == self.to.0 || self.from.1 == self.to.1
    }

    fn points(&self, diags: bool) -> HashSet<Point> {
        let mut points = HashSet::new();
        if self.from.0 == self.to.0 {
            for i in gen_range_incl(self.from.1, self.to.1) {
                points.insert(Point(self.from.0, i));
            }
        } else if self.from.1 == self.to.1 {
            for i in gen_range_incl(self.from.0, self.to.0) {
                points.insert(Point(i, self.from.1));
            }
        } else if diags {
            for (i, j) in
                gen_range_incl(self.from.0, self.to.0).zip(gen_range_incl(self.from.1, self.to.1))
            {
                points.insert(Point(i, j));
            }
        }
        points
    }
}

fn parse(filename: &str) -> Result<Vec<Line>> {
    let strings = parse_strings(filename)?;
    Ok(strings.iter().map(|x| Line::from_str(&*x)).collect())
}

fn find_intersection(filename: &str, with_diags: bool) -> Result<usize> {
    let lines = parse(filename)?;
    let mut double_points: HashSet<Point> = HashSet::new();
    for i in 0..lines.len() - 1 {
        for j in i + 1..lines.len() {
            let intersection = lines[i]
                .points(with_diags)
                .intersection(&lines[j].points(with_diags))
                .cloned()
                .collect();
            double_points = double_points.union(&intersection).cloned().collect();
        }
    }
    Ok(double_points.len())
}

fn q1(filename: &str) -> Result<usize> {
    find_intersection(filename, false)
}

fn q2(filename: &str) -> Result<usize> {
    find_intersection(filename, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(q1("./data/day5.txt").unwrap(), 5632);
        assert_eq!(q2("./data/day5.txt").unwrap(), 22213);
    }
}
