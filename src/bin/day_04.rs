use std::collections::HashMap;
use std::fs::read_to_string;

const VALID_NUMBERS: &str = "0123456789";
const VALID_HEX_CHARS: &str = "0123456789abcdef";
const ALLOWED_ECL: &str = "amb blu brn gry grn hzl oth";

fn check_byr(val: &str) -> bool {
    let v = val.parse::<usize>().unwrap_or(0);
    v >= 1020 && v <= 2002
}

fn check_iyr(val: &str) -> bool {
    let v = val.parse::<usize>().unwrap_or(0);
    v >= 2010 && v <= 2020
}

fn check_eyr(val: &str) -> bool {
    let v = val.parse::<usize>().unwrap_or(0);
    v >= 2020 && v <= 2030
}

fn check_hgt(val: &str) -> bool {
    if val.ends_with("in") {
        let v = val.trim_end_matches("in").parse::<usize>().unwrap_or(0);
        return v >= 59 && v <= 76;
    } else if val.ends_with("cm") {
        let v = val.trim_end_matches("cm").parse::<usize>().unwrap_or(0);
        return v >= 150 && v <= 193;
    }

    false
}

fn check_hcl(val: &str) -> bool {
    if val.len() != 7 || !val.starts_with('#') {
        return false;
    }

    let iter = val.chars().skip(1);
    iter.filter(|c| VALID_HEX_CHARS.contains(*c)).count() == 6
}

fn check_ecl(val: &str) -> bool {
    ALLOWED_ECL.contains(val)
}

fn check_pid(val: &str) -> bool {
    val.chars().filter(|c| VALID_NUMBERS.contains(*c)).count() == 9
}

pub fn main() {
    let buf = read_to_string("inputs/day_04.txt").unwrap();

    // 1. Split into passports (look for 2 newlines).
    // 2. Split each passport into fields ("a:1 b:1" becomes ["a:1", "b:2"]).
    // 3. Split each field into key and value ("a:1" becomes ("a", "1")).
    // 4. For each passport, create a hashmap of key-value pairs
    //    ([("a", "1"), ("b", "2")] becomes { "a": "1", "b": "2" }).
    // 5. Collect this into a vector of hashmaps.
    let passports = buf.split("\n\n").map(|p| {
        p.split_whitespace()
            .map(|f| {
                let vals: Vec<_> = f.split(':').collect();
                (vals[0], vals[1])
            })
            .collect::<HashMap<_, _>>()
    });

    let part1: Vec<_> = passports
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
}
