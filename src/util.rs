use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub(crate) fn parse_numbers(filename: &str) -> Result<Vec<usize>> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    let v: Vec<usize> = reader
        .lines()
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .collect();
    Ok(v)
}

//2832.8, 1463.5
//3090.0, 1403.6
