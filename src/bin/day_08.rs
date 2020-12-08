use std::collections::HashSet;
use std::fs::read_to_string;

fn execute(lines: &Vec<&str>) -> (i32, bool) {
    let mut accumulator: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();

    let mut idx: i32 = 0;

    while idx < lines.len() as i32 {
        let line = lines[idx as usize].clone();

        let mut parts = line.split_whitespace();
        let op = parts.next().unwrap();

        match op {
            "jmp" => {
                idx += parts.next().unwrap().parse::<i32>().unwrap() - 1;
            }
            "acc" => {
                accumulator += parts.next().unwrap().parse::<i32>().unwrap();
            }
            "nop" => (),
            _ => panic!(),
        };

        if !visited.insert(idx) {
            // We've seen this line before, it's a loop!
            break;
        }

        idx += 1;
    }

    (accumulator, idx == lines.len() as i32)
}

pub fn main() {
    let buf = read_to_string("inputs/day_08.txt").unwrap();
    let lines = buf.lines().collect::<Vec<_>>();

    let part1 = execute(&lines);

    println!("Part 1: {}", part1.0);

    // For each line, check if the op is `jmp` or `nop`. If so, swap the opcode
    // and re-execute the program.
    for idx in 0..lines.len() {
        if lines[idx].starts_with("acc") {
            continue;
        }

        let mut new_lines = lines.clone();
        let mut new_op = String::new();

        if new_lines[idx].starts_with("jmp") {
            let val = new_lines[idx].split_whitespace().last().unwrap();
            new_op = format!("nop {}", val);
        } else if new_lines[idx].starts_with("nop") {
            let val = new_lines[idx].split_whitespace().last().unwrap();
            new_op = format!("jmp {}", val);
        }

        new_lines[idx] = new_op.as_str();
        let result = execute(&new_lines);

        if result.1 {
            println!("Part 2: {}", result.0);
            return;
        }
    }
}
