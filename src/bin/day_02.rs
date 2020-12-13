use std::fs::read_to_string;

struct Password {
    a: usize,
    b: usize,
    letter: char,
    word: String,
}

fn part1(passwords: &[Password]) -> usize {
    passwords.iter().fold(0, |acc, p| {
        let count = p.word.matches(p.letter).count();

        acc + (count >= p.a && count <= p.b) as usize
    })
}

fn part2(passwords: &[Password]) -> usize {
    passwords.iter().fold(0, |acc, p| {
        let check_a = p.word.as_bytes()[p.a - 1] as char == p.letter;
        let check_b = p.word.as_bytes()[p.b - 1] as char == p.letter;

        acc + (check_a ^ check_b) as usize
    })
}

pub fn main() {
    let buf = read_to_string("inputs/day_02.txt").unwrap();

    let passwords: Vec<Password> = buf
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let rule_parts: Vec<&str> = parts[0].split(|c| c == '-' || c == ' ').collect();

            Password {
                a: rule_parts[0].parse().unwrap(),
                b: rule_parts[1].parse().unwrap(),
                letter: rule_parts[2].chars().next().unwrap(),
                word: parts[1].trim().to_string(),
            }
        })
        .collect();

    println!("Part 1: {}", part1(&passwords));
    println!("Part 2: {}", part2(&passwords));
}
