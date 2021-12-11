use crate::util::parse_strings;
use anyhow::Result;

#[derive(Debug)]
struct Code {
    unknowns: Vec<String>,
    knowns: [Option<String>; 10],
    input: [String; 4],
}

impl Code {
    fn from_str(s: &str) -> Self {
        let mut split = s.split(" | ");
        let unknowns = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(String::from)
            .collect();
        let mut i = split.next().unwrap().split_whitespace().map(String::from);
        let input = [
            i.next().unwrap(),
            i.next().unwrap(),
            i.next().unwrap(),
            i.next().unwrap(),
        ];
        Self {
            unknowns,
            knowns: Default::default(),
            input,
        }
    }

    fn resolve(&mut self) {
        self.resolve_1478();
        self.resolve_039();
        self.resolve_256();
    }

    fn resolve_1478(&mut self) {
        let removed_lengths = self.unknowns.clone();
        self.unknowns = removed_lengths
            .into_iter()
            .filter(|u| match u.len() {
                2 => {
                    self.knowns[1] = Some(u.to_string());
                    false
                }
                3 => {
                    self.knowns[7] = Some(u.to_string());
                    false
                }
                4 => {
                    self.knowns[4] = Some(u.to_string());
                    false
                }
                7 => {
                    self.knowns[8] = Some(u.to_string());
                    false
                }
                _ => true,
            })
            .collect();
    }

    fn resolve_039(&mut self) {
        let cf = self.knowns[1].clone().unwrap();
        let cf: Vec<char> = cf.chars().collect();
        // This essentially contains the code for 0,3 and 9. the only unknowns that have the 2
        // right ones (c,f in a standard cnfig)
        let unknowns_with_cf: Vec<String> = self
            .unknowns
            .clone()
            .into_iter()
            .filter(|x| x.contains(cf[0]) && x.contains(cf[1]))
            .collect();
        let three = unknowns_with_cf
            .iter()
            .find(|x| x.len() == 5)
            .cloned()
            .unwrap();
        let resolved: Vec<(String, usize)> = unknowns_with_cf
            .into_iter()
            .map(|x| {
                let num_diff = x
                    .chars()
                    .map(|c| if three.contains(c) { 0 } else { 1 })
                    .sum();
                let number = match num_diff {
                    0 => 3,
                    1 => 9,
                    2 => 0,
                    _ => unreachable!(),
                };
                (x, number)
            })
            .collect();
        self.unknowns = self
            .unknowns
            .clone()
            .into_iter()
            .filter(|x| resolved.iter().find(|(s, _)| s == x).is_none())
            .collect();
        resolved
            .into_iter()
            .for_each(|(coded, number)| self.knowns[number] = Some(coded));
    }

    fn resolve_256(&mut self) {
        let six = self
            .unknowns
            .iter()
            .find(|x| x.len() == 6)
            .cloned()
            .unwrap();
        let nine = self.knowns[9].clone().unwrap();
        let resolved: Vec<(String, usize)> = self
            .unknowns
            .clone()
            .into_iter()
            .filter(|x| *x != six)
            .map(|x| {
                let num_diff = x
                    .chars()
                    .map(|c| if nine.contains(c) { 0 } else { 1 })
                    .sum();
                let number = match num_diff {
                    0 => 5,
                    1 => 2,
                    _ => unreachable!(),
                };
                (x, number)
            })
            .collect();
        resolved
            .into_iter()
            .for_each(|(coded, number)| self.knowns[number] = Some(coded));
        self.knowns[6] = Some(six);
        self.unknowns = vec![];
    }

    fn try_decoding(&self, encoded: &str) -> Option<usize> {
        let mut options: Vec<(usize, String)> = self
            .knowns
            .iter()
            .enumerate()
            .filter(|(_, val)| val.is_some())
            .map(|(i, x)| (i, x.clone().unwrap()))
            .filter(|(_, x)| x.len() == encoded.len())
            .collect();
        for c in encoded.chars() {
            options = options.into_iter().filter(|(_, x)| x.contains(c)).collect();
        }
        if options.len() == 1 {
            Some(options[0].0)
        } else {
            None
        }
    }

    fn num_parsed(&self) -> usize {
        self.input
            .iter()
            .map(|encoded| self.try_decoding(encoded))
            .filter(Option::is_some)
            //.for_each(|x| { dbg!(x); })
            .count()
    }

    fn decode_input(&self) -> usize {
        let decoded: Vec<usize> = self
            .input
            .iter()
            .map(|x| self.try_decoding(x).unwrap())
            .collect();
        decoded[0] * 1000 + decoded[1] * 100 + decoded[2] * 10 + decoded[3]
    }
}

fn parse(filename: &str) -> Result<Vec<Code>> {
    let strings = parse_strings(filename)?;
    let result = strings.iter().map(|s| Code::from_str(s)).collect();
    Ok(result)
}

fn q1(filename: &str) -> Result<usize> {
    let mut coded = parse(filename)?;
    coded.iter_mut().for_each(Code::resolve_1478);
    let result = coded.iter().map(Code::num_parsed).sum();
    Ok(result)
}

fn q2(filename: &str) -> Result<usize> {
    let mut coded = parse(filename)?;
    coded.iter_mut().for_each(Code::resolve);
    let result = coded.iter().map(Code::decode_input).sum();
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        assert_eq!(q1("./data/day08.txt").unwrap(), 342);
        assert_eq!(q2("./data/day08.txt").unwrap(), 1068933);
    }
}
