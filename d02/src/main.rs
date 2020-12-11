#![feature(never_type)]

use std::io::BufRead;
use std::str::FromStr;

use anyhow::Error;

fn main() -> Result<(), Error> {
    let reader = std::io::BufReader::new(std::fs::File::open("input.list")?);
    let lines = reader
        .lines()
        .filter(|l| l.as_ref().map(|l| !l.trim().is_empty()).unwrap_or(true));
    let inputs = lines.map(|l| {
        l.map_err(Error::from)
            .and_then(|l| l.parse::<PwWithPolicy>().map_err(Error::from))
    });
    let inputs_vec: Vec<_> = inputs.collect::<Result<_, _>>()?;

    println!("part 1: {}", part1(&inputs_vec));

    println!("part 2: {}", part2(&inputs_vec));

    Ok(())
}

struct PwWithPolicy {
    letter: char,
    min: u64,
    max: u64,
    pw: String,
}
impl FromStr for PwWithPolicy {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_colon = s.split(": ");
        let policy = split_colon.next().expect("unreachable");
        let pw = split_colon
            .next()
            .ok_or(anyhow::anyhow!("missing pw"))?
            .into();
        let letter = policy
            .chars()
            .next_back()
            .ok_or(anyhow::anyhow!("empty policy"))?;
        let mut range_split = policy.split(|c: char| !c.is_digit(10));
        let min = range_split.next().expect("unreachable").parse()?;
        let max = range_split
            .next()
            .ok_or(anyhow::anyhow!("range missing upper bound"))?
            .parse()?;
        Ok(PwWithPolicy {
            letter,
            min,
            max,
            pw,
        })
    }
}

fn is_valid_1(pw: &PwWithPolicy) -> bool {
    (pw.min..=pw.max).contains(&(pw.pw.chars().filter(|c| c == &pw.letter).count() as u64))
}

fn part1(inputs: &[PwWithPolicy]) -> u64 {
    inputs.into_iter().filter(|pw| is_valid_1(*pw)).count() as u64
}

fn is_valid_2(pw: &PwWithPolicy) -> bool {
    (pw.pw.chars().skip(pw.min as usize - 1).next().unwrap() == pw.letter)
        != (pw.pw.chars().skip(pw.max as usize - 1).next().unwrap() == pw.letter)
}

fn part2(inputs: &[PwWithPolicy]) -> u64 {
    inputs.into_iter().filter(|pw| is_valid_2(*pw)).count() as u64
}
