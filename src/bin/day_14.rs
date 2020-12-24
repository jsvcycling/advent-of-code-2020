use std::collections::HashMap;
use std::fs::read_to_string;

fn parse_mask(input: &str) -> (u64, u64) {
    let mask = input.split_whitespace().last().unwrap();

    let mask_1 = mask.replace("X", "0");
    let mask_0 = mask.replace("1", "X").replace("0", "1").replace("X", "0");

    let mask_1 = u64::from_str_radix(&mask_1, 2).unwrap();
    let mask_0 = u64::from_str_radix(&mask_0, 2).unwrap();

    (mask_1, mask_0)
}

fn parse_mem(input: &str) -> (u64, u64) {
    let parts = input
        .split(|c| c == '[' || c == ']' || c == ' ')
        .collect::<Vec<_>>();

    let addr = parts[1].parse::<u64>().unwrap();
    let val = parts[4].parse::<u64>().unwrap();

    (addr, val)
}

fn apply_part1(mask: (u64, u64), input: u64) -> u64 {
    let tmp_0 = mask.0 | input;
    let tmp_1 = mask.1 & tmp_0;
    tmp_0 & !tmp_1
}

fn apply_part2(mask: &str, input: u64) -> Vec<u64> {
    let mut addrs = Vec::new();

    // Step 1. Handle `1` bits from the mask.
    let mask_u64 = mask.replace("X", "0");
    let mask_u64 = u64::from_str_radix(&mask_u64, 2).unwrap();
    let input = input | mask_u64;

    // Step 2. Handle `X` bits from the mask.
    let floating = mask.match_indices("X").map(|(i, _)| i).collect::<Vec<_>>();

    for i in 0..(2u64.pow(floating.len() as u32)) {
        let mut mask_xor = 0;

        floating.iter().enumerate().for_each(|(idx, pos)| {
            if (i & (2u64.pow(idx as u32))) != 0 {
                mask_xor |= 1 << (36 - pos - 1);
            }
        });

        addrs.push(input ^ mask_xor);
    }

    addrs
}

fn part1(lines: &[&str]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = (0, 0);

    for line in lines {
        if line.starts_with("mask") {
            mask = parse_mask(line);
        } else if line.starts_with("mem") {
            let m = parse_mem(line);
            let val = apply_part1(mask, m.1);
            mem.entry(m.0).and_modify(|v| *v = val).or_insert(val);
        }
    }

    mem.values().sum()
}

fn part2(lines: &[&str]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = "";

    for line in lines {
        if line.starts_with("mask") {
            mask = line.split_whitespace().last().unwrap();
        } else if line.starts_with("mem") {
            let m = parse_mem(line);
            let addrs = apply_part2(mask, m.0);

            for addr in addrs {
                mem.entry(addr).and_modify(|v| *v = m.1).or_insert(m.1);
            }
        }
    }

    mem.values().sum()
}

pub fn main() {
    let buf = read_to_string("inputs/day_14.txt").unwrap();
    let lines = buf.lines().collect::<Vec<_>>();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}
