use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub fn main() {
    let buf = read_to_string("inputs/day_06.txt").unwrap();
    let groups: Vec<_> = buf.split("\n\n").collect();

    let part1 = groups.iter().fold(0, |acc, g| {
        let mut ids = g.chars().collect::<HashSet<_>>();
        ids.remove(&'\n');
        acc + ids.len()
    });

    let part2 = groups.iter().fold(0, |acc, g| {
        let mut ids = HashMap::new();
        let persons: Vec<_> = g.split_whitespace().collect();

        persons.iter().for_each(|p| {
            p.chars().for_each(|c| {
                ids.entry(c).and_modify(|v| *v += 1).or_insert(1);
            });
        });

        acc + ids.values().filter(|v| **v == persons.len()).count()
    });

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
