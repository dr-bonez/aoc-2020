#![feature(never_type)]

use std::io::BufRead;

use anyhow::Error;

fn main() -> Result<(), Error> {
    let reader = std::io::BufReader::new(std::fs::File::open("input.list")?);
    let lines = reader
        .lines()
        .filter(|l| l.as_ref().map(|l| !l.trim().is_empty()).unwrap_or(true));
    let lines: Vec<_> = lines.collect::<Result<_, _>>()?;

    println!("part 1: {}", trees(&lines, 3, 1));

    println!(
        "part 2: {}",
        trees(&lines, 1, 1)
            * trees(&lines, 3, 1)
            * trees(&lines, 5, 1)
            * trees(&lines, 7, 1)
            * trees(&lines, 1, 2)
    );

    Ok(())
}

fn trees(lines: &[String], right: usize, down: usize) -> usize {
    let mut idx = 0;
    let mut trees = 0;
    for line_num in 0..lines.len() {
        if line_num % down != 0 {
            continue;
        }
        let byte_line = lines[line_num].trim().as_bytes();
        if byte_line[idx % byte_line.len()] == b'#' {
            trees += 1;
        }
        idx += right;
    }
    trees
}
