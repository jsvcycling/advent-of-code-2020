use std::io::prelude::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Result};

const VALID_NUMBERS: &str = "0123456789";
const VALID_HEX_CHARS: &str = "01234567890abcdef";

fn check_byr(val: &str) -> bool {
    if let Ok(v) = val.parse::<usize>() {
        return v >= 1020 && v <= 2002;
    }

    false
}

fn check_iyr(val: &str) -> bool {
    if let Ok(v) = val.parse::<usize>() {
        return v >= 2010 && v <= 2020;
    }

    false
}

fn check_eyr(val: &str) -> bool {
    if let Ok(v) = val.parse::<usize>() {
        return v >= 2020 && v <= 2030;
    }

    false
}

fn check_hgt(val: &str) -> bool {
    if val.ends_with("in") {
        if let Ok(v) = val.trim_end_matches("in").parse::<usize>() {
            return v >= 59 && v <= 76;
        }
    } else if val.ends_with("cm") {
        if let Ok(v) = val.trim_end_matches("cm").parse::<usize>() {
            return v >= 150 && v <= 193;
        }
    }

    false
}

fn check_hcl(val: &str) -> bool {
    if val.len() == 7 && val.starts_with('#') {
        let mut iter = val.chars();
        iter.next();
        return iter.filter(|c| VALID_HEX_CHARS.contains(*c)).count() == 6;
    }

    false
}

fn check_ecl(val: &str) -> bool {
    val == "amb"
        || val == "blu"
        || val == "brn"
        || val == "gry"
        || val == "grn"
        || val == "hzl"
        || val == "oth"
}

fn check_pid(val: &str) -> bool {
    val.chars().filter(|c| VALID_NUMBERS.contains(*c)).count() == 9
}

pub fn main() -> Result<()> {
    let mut f = BufReader::new(File::open("inputs/day_04.txt")?);

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    // 1. Split into passports (look for 2 newlines).
    // 2. Split each passport into fields ("a:1 b:1" becomes ["a:1", "b:2"]).
    // 3. Split each field into key and value ("a:1" becomes ("a", "1")).
    // 4. For each passport, create a hashmap of key-value pairs
    //    ([("a", "1"), ("b", "2")] becomes { "a": "1", "b": "2" }).
    // 5. Collect this into a vector of hashmaps.
    let passports: Vec<_> = buf
        .split("\n\n")
        .map(|p| {
            p.split_whitespace()
                .map(|f| {
                    let mut vals = f.split(':');
                    (vals.next().unwrap(), vals.next().unwrap())
                })
                .collect::<HashMap<_, _>>()
        })
        .collect();

    let part1: Vec<_> = passports
        .iter()
        .filter(|p| {
            p.contains_key("byr")
                && p.contains_key("iyr")
                && p.contains_key("eyr")
                && p.contains_key("hgt")
                && p.contains_key("hcl")
                && p.contains_key("ecl")
                && p.contains_key("pid")
        })
        .collect();

    let part2 = part1
        .iter()
        .filter(|p| {
            check_byr(p.get("byr").unwrap())
                && check_iyr(p.get("iyr").unwrap())
                && check_eyr(p.get("eyr").unwrap())
                && check_hgt(p.get("hgt").unwrap())
                && check_hcl(p.get("hcl").unwrap())
                && check_ecl(p.get("ecl").unwrap())
                && check_pid(p.get("pid").unwrap())
        })
        .count();

    println!("Part 1: {}", part1.len());
    println!("Part 2: {}", part2);

    Ok(())
}
