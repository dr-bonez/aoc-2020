#![feature(never_type)]

use std::io::BufRead;
use std::str::FromStr;

use anyhow::*;

fn main() -> Result<(), Error> {
    let reader = std::io::BufReader::new(std::fs::File::open("input.list")?);
    let lines = reader.lines();
    let mut inputs = Vec::new();
    let mut cur = Passport::default();
    for line in lines {
        let line = line?;
        if line.trim().is_empty() {
            let add = std::mem::replace(&mut cur, Passport::default());
            inputs.push(add);
        }
        cur.input(&line)?;
    }
    inputs.push(cur);

    println!("part 1: {}", inputs.iter().filter(|p| p.is_valid()).count());

    eprintln!(
        "{:?}",
        inputs
            .iter()
            .map(Passport::validate_fields)
            .collect::<Vec<_>>()
    );
    println!(
        "part 2: {}",
        inputs
            .iter()
            .filter(|p| p.validate_fields().is_ok())
            .count()
    );

    Ok(())
}

struct Height {
    amount: usize,
    unit: Unit,
}
impl FromStr for Height {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (amount, unit) = s.split_at(
            s.find(|c: char| !c.is_digit(10))
                .ok_or(anyhow!("no unit"))?,
        );
        Ok(Height {
            amount: amount.parse()?,
            unit: match unit {
                "in" => Unit::In,
                "cm" => Unit::Cm,
                _ => bail!("invalid unit"),
            },
        })
    }
}

enum Unit {
    In,
    Cm,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}
impl FromStr for Color {
    type Err = Error;
    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        s = s.strip_prefix("#").ok_or(anyhow!("missing #"))?;
        Ok(Color {
            r: u8::from_str_radix(&s[..2], 16)?,
            g: u8::from_str_radix(&s[2..4], 16)?,
            b: u8::from_str_radix(&s[4..], 16)?,
        })
    }
}

#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}
impl Passport {
    fn input(&mut self, line: &str) -> Result<(), Error> {
        for item in line.split_whitespace() {
            let mut split = item.split(":");
            let key = split.next().expect("unreachable");
            let val = split.next().ok_or(anyhow!("invalid batch item"))?;
            match key {
                "byr" => self.byr = Some(val.into()),
                "iyr" => self.iyr = Some(val.into()),
                "eyr" => self.eyr = Some(val.into()),
                "hgt" => self.hgt = Some(val.into()),
                "hcl" => self.hcl = Some(val.into()),
                "ecl" => self.ecl = Some(val.into()),
                "pid" => self.pid = Some(val.into()),
                "cid" => self.cid = Some(val.into()),
                _ => (),
            }
        }
        Ok(())
    }
    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
    fn validate_fields(&self) -> Result<(), Error> {
        let byr: usize = self.byr.as_ref().ok_or(anyhow!("missing byr"))?.parse()?;
        ensure!((1920..=2002).contains(&byr), "invalid byr");
        let iyr: usize = self.iyr.as_ref().ok_or(anyhow!("missing iyr"))?.parse()?;
        ensure!((2010..=2020).contains(&iyr), "invalid iyr");
        let eyr: usize = self.eyr.as_ref().ok_or(anyhow!("missing eyr"))?.parse()?;
        ensure!((2020..=2030).contains(&eyr), "invalid eyr");
        let hgt: Height = self.hgt.as_ref().ok_or(anyhow!("missing hgt"))?.parse()?;
        match hgt.unit {
            Unit::In => {
                ensure!((59..=76).contains(&hgt.amount), "invalid height");
            }
            Unit::Cm => {
                ensure!((150..=193).contains(&hgt.amount), "invalid height");
            }
        }
        self.hcl
            .as_ref()
            .ok_or(anyhow!("missing hcl"))?
            .parse::<Color>()?;
        ensure!(
            match self.ecl.as_ref().ok_or(anyhow!("missing ecl"))?.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            "invalid ecl"
        );
        let pid = self.pid.as_ref().ok_or(anyhow!("missing ecl"))?.as_str();
        ensure!(
            pid.len() == 9 && pid.chars().all(|c: char| c.is_digit(10)),
            "invalid pid"
        );

        Ok(())
    }
}
