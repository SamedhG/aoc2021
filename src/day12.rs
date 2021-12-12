use crate::util::parse_strings;
use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;
type Map = HashMap<String, Vec<String>>;

fn parse(filename: &str) -> Result<Map> {
    let lines = parse_strings(filename)?;
    let mut map: Map = HashMap::new();
    for line in lines {
        let mut iter = line.split('-');
        let from = String::from(iter.next().unwrap());
        let to = String::from(iter.next().unwrap());
        match map.get_mut(&from) {
            Some(list) => list.push(to.clone()),
            None => {
                map.insert(from.clone(), vec![to.clone()]);
            }
        }

        match map.get_mut(&to) {
            Some(list) => list.push(from.clone()),
            None => {
                map.insert(to.clone(), vec![from.clone()]);
            }
        }
    }
    Ok(map)
}

fn is_big_cave(cave: &str) -> bool {
    cave.chars().nth(0).unwrap().is_uppercase()
}

// Assuming that an endlessly long path cannot exist
fn count_paths(map: &Map, from: &str, seen_smalls: &HashSet<String>) -> usize {
    if from == "end" {
        1
    } else {
        map.get(from)
            .unwrap()
            .iter()
            .filter(|cave| !seen_smalls.contains(*cave))
            .map(|cave| {
                if is_big_cave(cave) {
                    count_paths(&map, cave, &seen_smalls)
                } else {
                    let mut new_seen_smalls = seen_smalls.clone();
                    new_seen_smalls.insert(cave.clone());
                    count_paths(&map, cave, &new_seen_smalls)
                }
            })
            .sum()
    }
}

fn q1(filename: &str) -> Result<usize> {
    let map = parse(filename)?;
    let seen_smalls = HashSet::from([String::from("start")]);
    let num_paths = count_paths(&map, "start", &seen_smalls);
    Ok(num_paths)
}
// Assuming that an endlessly long path cannot exist
fn count_paths_double(
    map: &Map,
    from: &String,
    seen_smalls: &HashSet<String>,
    double_counted_small: &Option<String>,
) -> usize {
    if from == "end" {
        1
    } else {
        map.get(from)
            .unwrap()
            .iter()
            .filter(|cave| *cave != "start")
            .map(|cave| {
                if is_big_cave(cave) {
                    count_paths_double(&map, cave, &seen_smalls, &double_counted_small)
                } else if seen_smalls.contains(cave) {
                    match double_counted_small {
                        Some(_) => 0,
                        None => {
                            let double_count = Some(cave.clone());
                            count_paths_double(&map, cave, &seen_smalls, &double_count)
                        }
                    }
                } else {
                    let mut new_seen_smalls = seen_smalls.clone();
                    new_seen_smalls.insert(cave.clone());
                    count_paths_double(&map, cave, &new_seen_smalls, &double_counted_small)
                }
            })
            .sum()
    }
}

fn q2(filename: &str) -> Result<usize> {
    let map = parse(filename)?;
    let start = String::from("start");
    let seen_smalls = HashSet::from([start.clone()]);
    let num_paths = count_paths_double(&map, &start, &seen_smalls, &None);
    Ok(num_paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(q1("./data/day12.txt").unwrap(), 3576);
        assert_eq!(q2("./data/day12.txt").unwrap(), 84271);
    }
}
