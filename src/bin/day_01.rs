use std::io::prelude::*;

use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Result};

fn part1(amounts: &Vec<u32>) -> Result<u32> {
    for x in 0..amounts.len() {
        for y in (x + 1)..amounts.len() {
            if amounts[x] + amounts[y] == 2020 {
                return Ok(amounts[x] * amounts[y]);
            }
        }
    }

    Err(Error::from(ErrorKind::NotFound))
}

fn part2(amounts: &Vec<u32>) -> Result<u32> {
    for x in 0..amounts.len() {
        for y in (x + 1)..amounts.len() {
            for z in (y + 1)..amounts.len() {
                if amounts[x] + amounts[y] + amounts[z] == 2020 {
                    return Ok(amounts[x] * amounts[y] * amounts[z]);
                }
            }
        }
    }

    Err(Error::from(ErrorKind::NotFound))
}

pub fn main() -> Result<()> {
    let f = BufReader::new(File::open("inputs/day_01.txt")?);

    let mut amounts = f
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    amounts.sort();

    println!("Part 1: {}", part1(&amounts)?);
    println!("Part 2: {}", part2(&amounts)?);
    Ok(())
}
