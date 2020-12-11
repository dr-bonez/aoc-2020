#![feature(never_type)]

use std::io::BufRead;

use anyhow::Error;

fn main() -> Result<(), Error> {
    let reader = std::io::BufReader::new(std::fs::File::open("input.list")?);
    let lines = reader
        .lines()
        .filter(|l| l.as_ref().map(|l| !l.trim().is_empty()).unwrap_or(true));
    let inputs = lines.map(|l| {
        l.map_err(Error::from)
            .and_then(|l| l.parse::<u64>().map_err(Error::from))
    });

    println!("part 1: {}", part1());

    println!("part 2: {}", part2());

    Ok(())
}

fn part1() -> ! {
    unimplemented!()
}

fn part2() -> ! {
    unimplemented!()
}
