#![feature(never_type)]

use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::io::BufRead;

use anyhow::Error;

fn main() -> Result<(), Error> {
    let reader = std::io::BufReader::new(std::fs::File::open("input.list")?);
    let lines = reader
        .lines()
        .filter(|l| l.as_ref().map(|l| !l.trim().is_empty()).unwrap_or(true));
    let inputs = lines.map(|l| {
        l.map_err(Error::from).and_then(|l| {
            l.chars()
                .map(|c| c.try_into())
                .collect::<Result<Vec<Partition>, _>>()
        })
    });
    let input_vec: Vec<_> = inputs.collect::<Result<_, _>>()?;

    println!(
        "part 1: {}",
        input_vec
            .iter()
            .map(|bsp| seat_number(128, 8, bsp))
            .map(|(row, column)| seat_id(row, column))
            .max()
            .ok_or(anyhow::anyhow!("empty list"))?
    );

    println!("part 2: {:?}", {
        let mut used = vec![false; 128 * 8];
        let mut candidates = HashSet::new();
        for seat_id in input_vec
            .iter()
            .map(|bsp| seat_number(128, 8, bsp))
            .map(|(row, column)| seat_id(row, column))
        {
            used[seat_id] = true;
            if used.get(seat_id + 2).map(|t| *t).unwrap_or(false) && !used[seat_id + 1] {
                candidates.insert(seat_id + 1);
            }
            if seat_id >= 2 && used[seat_id - 2] && !used[seat_id - 1] {
                candidates.insert(seat_id - 1);
            }
            candidates.remove(&seat_id);
        }
        candidates
    });

    Ok(())
}

enum Partition {
    F,
    B,
    L,
    R,
}
impl TryFrom<char> for Partition {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'F' | 'f' => Partition::F,
            'B' | 'b' => Partition::B,
            'L' | 'l' => Partition::L,
            'R' | 'r' => Partition::R,
            _ => anyhow::bail!("invalid partition: {}", value),
        })
    }
}

fn seat_id(row: usize, column: usize) -> usize {
    row * 8 + column
}

fn seat_number(rows: usize, columns: usize, bsp: &[Partition]) -> (usize, usize) {
    let mut min_row: usize = 0;
    let mut max_row: usize = rows;
    let mut min_column: usize = 0;
    let mut max_column: usize = columns;
    for part in bsp {
        match part {
            Partition::F => max_row = min_row + ((max_row - min_row) / 2),
            Partition::B => min_row = max_row - ((max_row - min_row) / 2),
            Partition::L => max_column = min_column + ((max_column - min_column) / 2),
            Partition::R => min_column = max_column - ((max_column - min_column) / 2),
        }
    }
    (min_row, min_column)
}
