use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub(crate) fn parse_numbers(filename: &str, radix: u32) -> Result<Vec<usize>> {
    let v = parse_strings(filename)?;
    Ok(v.iter()
        .map(|l| usize::from_str_radix(l, radix).unwrap())
        .collect())
}

pub(crate) fn parse_numbers_comma(filename: &str) -> Result<Vec<isize>> {
    let mut s = std::fs::read_to_string(filename)?;
    s.pop();
    Ok(s.split(',')
        .map(|n| isize::from_str_radix(n, 10).unwrap())
        .collect())
}

pub(crate) fn parse_strings(filename: &str) -> Result<Vec<String>> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    let v: Vec<String> = reader.lines().map(Result::unwrap).collect();
    Ok(v)
}

//2832.8, 1463.5
//3090.0, 1403.6
