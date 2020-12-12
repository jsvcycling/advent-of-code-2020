use std::cmp::Ordering;
use std::fs::read_to_string;

fn part1(values: &[u64]) -> u64 {
    'outer: for idx in 25..values.len() {
        for i in (1..=25).rev() {
            for j in ((i + 1)..=25).rev() {
                if values[idx - i] + values[idx - j] == values[idx] {
                    continue 'outer;
                }
            }
        }

        return values[idx];
    }

    0
}

fn part2(values: &[u64], target: &u64) -> u64 {
    for start_idx in 0..values.len() {
        let mut min = u64::MAX;
        let mut max = u64::MIN;
        let mut sum = 0;

        for val in values.iter().skip(start_idx) {
            match (val + sum).cmp(target) {
                Ordering::Less => {
                    // If the new total is still less than the target, append
                    // this value the list and move onto the next value.
                    min = u64::min(min, *val);
                    max = u64::max(max, *val);
                    sum += *val;
                    continue;
                }
                Ordering::Greater => {
                    // If we're going to pass the target with this value, we
                    // know this sequence won't work so stop and move onto the
                    // next one.
                    break;
                }
                _ => {
                    // We found the right sequence!
                    min = u64::min(min, *val);
                    max = u64::max(max, *val);
                    return min + max;
                }
            }
        }
    }

    0
}

pub fn main() {
    let buf = read_to_string("inputs/day_09.txt").unwrap();
    let values: Vec<_> = buf.lines().map(|l| l.parse::<u64>().unwrap()).collect();

    let part1 = part1(&values);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2(&values, &part1));
}
