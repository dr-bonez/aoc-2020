#![feature(never_type)]

use std::io::BufRead;
use std::collections::HashSet;

use anyhow::Error;

fn main() -> Result<(), Error> {
    let reader = std::io::BufReader::new(std::fs::File::open("input.list")?);
    let lines = reader.lines().filter(|l| l.as_ref().map(|l| !l.trim().is_empty()).unwrap_or(true));
    let inputs = lines.map(|l| l.map_err(Error::from).and_then(|l| l.parse::<u64>().map_err(Error::from)));

    let inputs_set: HashSet<_> = inputs.collect::<Result<_, _>>()?;

    println!("part 1: {:?}", part1(&inputs_set, 2020));

    println!("part 2: {:?}", part2(&inputs_set, 2020));

    Ok(())
}

fn find_pair(inputs_set: &HashSet<u64>, sum: u64) -> Option<(u64, u64)> {
    for input in inputs_set {
        if *input > sum {
            continue;
        }
        if inputs_set.contains(&(sum - input)) {
            return Some((*input, sum - input))
        }
    }
    None
}

fn part1(inputs_set: &HashSet<u64>, sum: u64) -> Option<u64> {
    find_pair(inputs_set, sum).map(|(a, b)| a * b)
}

fn part2(inputs_set: &HashSet<u64>, sum: u64) -> Option<u64> {
    for input in inputs_set {
        if let Some((a, b)) = find_pair(inputs_set, sum - input) {
            return Some(input * a * b)
        }
    }
    None
}