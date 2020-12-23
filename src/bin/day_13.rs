use std::fs::read_to_string;

fn part1(target: &usize, routes: &[usize]) -> usize {
    let mut min_diff = usize::MAX;
    let mut min_id = 0;

    routes.iter().filter(|r| **r != 0).for_each(|r| {
        // Find the first instance of `r` that is above the target.
        let val = r * ((target / r) + 1);

        if (val - target) < min_diff {
            min_diff = val - target;
            min_id = *r;
        }
    });

    min_id * min_diff
}

// Based on Reddit (uses Chinese remainder theorem).
fn part2(routes: &[usize]) -> usize {
    let mut solution = 0;
    let mut lcd = 1;

    routes
        .iter()
        .enumerate()
        .filter(|(_, v)| **v != 0)
        .for_each(|(offset, route)| {
            while (solution + offset) % route != 0 {
                solution += lcd;
            }

            lcd *= route;
        });

    solution
}

fn main() {
    let buf = read_to_string("inputs/day_13.txt").unwrap();
    let lines = buf.lines().collect::<Vec<_>>();

    let target = lines[0].parse::<usize>().unwrap();
    let routes = lines[1]
        .split(',')
        .map(|r| r.parse::<usize>().unwrap_or(0))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&target, &routes));
    println!("Part 2: {}", part2(&routes));
}
