use std::fs::read_to_string;

fn part1(target: &usize, routes: &[&str]) -> usize {
    let routes = routes
        .iter()
        .filter(|c| **c != "x")
        .map(|r| r.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut min_val = usize::MAX;
    let mut min_id = 0;

    for route in routes {
        let iters = target / route;
        let val = route * (iters + 1);
        let diff = val - target;

        if diff < min_val {
            min_val = diff;
            min_id = route;
        }
    }

    min_id * min_val
}

// Based on Reddit (uses Chinese remainder theorem).
fn part2(routes: &[&str]) -> usize {
    let routes = routes
        .iter()
        .enumerate()
        .filter(|(_, v)| **v != "x")
        .map(|(k, v)| (k, v.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();

    let mut solution = 0;
    let mut lcd = 1;

    routes.into_iter().for_each(|(offset, route)| {
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
    let routes = lines[1].split(',').collect::<Vec<_>>();

    println!("Part 1: {}", part1(&target, &routes));
    println!("Part 2: {}", part2(&routes));
}
