use std::io::prelude::*;

use std::fs::File;
use std::io::{BufReader, Result};

fn compute(lines: &Vec<String>, move_x: usize, move_y: usize) -> Result<usize> {
    let mut num_trees = 0;
    let mut pos = (0, 0);

    let line_length = lines[0].len();

    while pos.1 + move_y < lines.len() {
        pos.0 += move_x;
        pos.1 += move_y;

        if pos.0 >= line_length {
            pos.0 -= line_length;
        }

        if lines[pos.1].as_bytes()[pos.0] as char == '#' {
            num_trees += 1;
        }
    }

    Ok(num_trees)
}

pub fn main() -> Result<()> {
    let f = BufReader::new(File::open("inputs/day_03.txt")?);

    let lines: Vec<String> = f.lines().map(|l| l.unwrap()).collect();

    let part1 = compute(&lines, 3, 1)?;

    println!("Part 1: {}", part1);

    let part_2a = compute(&lines, 1, 1)?;
    let part_2b = compute(&lines, 5, 1)?;
    let part_2c = compute(&lines, 7, 1)?;
    let part_2d = compute(&lines, 1, 2)?;

    println!("Part 2: {}", part_2a * part1 * part_2b * part_2c * part_2d);

    Ok(())
}
