use std::io::BufRead;
use std::collections::VecDeque;

use anyhow::Error;

fn main() -> Result<(), Error> {
    let rdr = std::io::BufReader::new(std::fs::File::open("input.list")?);
    let mut adapters: VecDeque<u64> = rdr
        .lines()
        .map(|l| l.map_err(Error::from))
        .filter(|l| !l.as_ref().map(|l| l.is_empty()).unwrap_or(true))
        .map(|l| l.and_then(|l| l.parse::<u64>().map_err(Error::from)))
        .collect::<Result<_, _>>()?;

    adapters.make_contiguous().sort_unstable();
    let phone = adapters[adapters.len() - 1] + 3;
    adapters.push_back(phone);
    let (_, one, three): (u64, u64, u64) = adapters
        .iter()
        .map(|x| *x)
        .fold((0, 0, 0), |acc, x| {
            let diff = x - acc.0;
            (
                x,
                if diff == 1 { acc.1 + 1 } else { acc.1 },
                if diff == 3 { acc.2 + 1 } else { acc.2 },
            )
        });

    println!("part 1: {}", one * three);

    adapters.push_front(0);

    println!("part 2: {}", count(adapters.make_contiguous()));

    Ok(())
}

fn count(adapters: &[u64]) -> u64 {
    let mut multiplier = vec![0; adapters.len()];
    multiplier[0] = 1;
    for idx in 0..adapters.len() {
        for idx2 in (idx + 1)..adapters.len() {
            if adapters[idx2] - adapters[idx] > 3 {
                break
            } else {
                multiplier[idx2] += multiplier[idx];
            }
        }
    }

    multiplier[multiplier.len() - 1]
}