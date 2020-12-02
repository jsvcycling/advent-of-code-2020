use std::io::prelude::*;

use std::fs::File;
use std::io::{BufReader, Result};

struct Password {
    a: usize,
    b: usize,
    letter: char,
    word: String,
}

fn part1(passwords: &Vec<Password>) -> usize {
    passwords
        .iter()
        .map(|p| {
            let count = p.word.matches(p.letter).count();

            count >= p.a && count <= p.b
        })
        .filter(|v| *v == true)
        .count()
}

fn part2(passwords: &Vec<Password>) -> usize {
    passwords
        .iter()
        .map(|p| {
            let check_a = p.word.as_bytes()[p.a - 1] as char == p.letter;
            let check_b = p.word.as_bytes()[p.b - 1] as char == p.letter;

            // Rust doesn't have an xor operator so here we go...
            (check_a || check_b) && !(check_a && check_b)
        })
        .filter(|v| *v == true)
        .count()
}

pub fn main() -> Result<()> {
    let f = BufReader::new(File::open("inputs/day_02.txt")?);

    let passwords: Vec<Password> = f
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let parts: Vec<&str> = line.split(':').collect();
            let rule_parts: Vec<&str> = parts[0].split(|c| c == '-' || c == ' ').collect();

            Password {
                a: rule_parts[0].parse().unwrap(),
                b: rule_parts[1].parse().unwrap(),
                letter: rule_parts[2].chars().next().unwrap(),
                word: String::from(parts[1].trim()),
            }
        })
        .collect();

    println!("Part 1: {}", part1(&passwords));
    println!("Part 2: {}", part2(&passwords));

    Ok(())
}
