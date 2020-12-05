use std::io::prelude::*;

use std::fs::File;
use std::io::{BufReader, Result};

pub fn main() -> Result<()> {
    let f = BufReader::new(File::open("inputs/day_05.txt")?);
    let lines: Vec<String> = f.lines().map(|l| l.unwrap()).collect();

    let mut ids: Vec<i32> = lines
        .iter()
        .map(|line| {
            let mut row_range = (0, 127);
            let mut seat_range = (0, 7);

            line.chars().for_each(|c| {
                match c {
                    'F' => {
                        row_range.1 = (row_range.1 + row_range.0) / 2;
                    }
                    'B' => {
                        row_range.0 = (row_range.1 + row_range.0 + 1) / 2;
                    }
                    'L' => {
                        seat_range.1 = (seat_range.1 + seat_range.0) / 2;
                    }
                    'R' => {
                        seat_range.0 = (seat_range.1 + seat_range.0 + 1) / 2;
                    }
                    _ => panic!("Received: {:?}", c),
                };
            });

            row_range.0 * 8 + seat_range.0
        })
        .collect();

    ids.sort();

    println!("Part 1: {}", ids.iter().max().unwrap());

    // We can do this more efficiently using a BSP again but we'll just do it
    // linearly. This will run in O(n) which isn't terrible for this case (a BSP
    // would run in O(log n) by comparison).
    for (idx, val) in ids.iter().enumerate() {
        if ids[idx + 1] != val + 1 {
            println!("Part 2: {}", val + 1);
            break;
        }
    }

    Ok(())
}
