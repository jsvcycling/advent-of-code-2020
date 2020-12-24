use std::fs::read_to_string;

fn part1(lines: &[usize]) -> usize {
    let mut num_1_diff = 0;
    let mut num_3_diff = 0;

    for idx in 1..lines.len() {
        match lines[idx] - lines[idx - 1] {
            1 => num_1_diff += 1,
            3 => num_3_diff += 1,
            _ => (),
        }
    }

    num_1_diff * num_3_diff
}

fn part2(lines: &[usize]) -> usize {
    let mut slices = Vec::new();
    let mut curr_slice = Vec::new();

    // Walk through the list and generate combinatorial slices, splitting
    // wherever there is a difference of 3.
    lines.windows(2).for_each(|w| {
        match w[1] - w[0] {
            1 => curr_slice.push(w[0]),
            3 => {
                // Split off.
                curr_slice.push(w[0]);
                slices.push(curr_slice.clone());
                curr_slice = Vec::new();
            }
            _ => (),
        }
    });

    // For each slice, determine the number of possible conbinations.
    slices
        .iter()
        .map(|s| match s.len() {
            1 => 1,
            2 => 1,
            3 => 2,
            4 => 4,
            5 => 7,
            _ => panic!(),
        })
        .product()
}

pub fn main() {
    let buf = read_to_string("inputs/day_10.txt").unwrap();
    let mut lines = buf
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let wall_jolts = 0;
    let device_jolts = lines.iter().max().unwrap() + 3;

    lines.push(wall_jolts);
    lines.push(device_jolts);

    lines.sort();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}
