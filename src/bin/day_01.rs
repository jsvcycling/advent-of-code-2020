use std::fs::read_to_string;

fn part1(amounts: &Vec<u32>) -> u32 {
    for x in 0..amounts.len() {
        for y in (x + 1)..amounts.len() {
            if amounts[x] + amounts[y] == 2020 {
                return amounts[x] * amounts[y];
            }
        }
    }

    0
}

fn part2(amounts: &Vec<u32>) -> u32 {
    for x in 0..amounts.len() {
        for y in (x + 1)..amounts.len() {
            for z in (y + 1)..amounts.len() {
                if amounts[x] + amounts[y] + amounts[z] == 2020 {
                    return amounts[x] * amounts[y] * amounts[z];
                }
            }
        }
    }

    0
}

pub fn main() {
    let mut amounts = read_to_string("inputs/day_01.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    amounts.sort();

    println!("Part 1: {}", part1(&amounts));
    println!("Part 2: {}", part2(&amounts));
}
