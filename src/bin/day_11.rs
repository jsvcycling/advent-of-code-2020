use std::fs::read_to_string;

type Grid = Vec<Vec<Option<bool>>>;

fn check_seat(grid: &Grid, x: i64, y: i64) -> Option<bool> {
    *grid
        .get(y as usize)
        .and_then(|g| g.get(x as usize))
        .unwrap_or(&None)
}

fn count_occupied(grid: &Grid) -> usize {
    grid.iter()
        .map(|y| y.iter().map(|x| x.unwrap_or(false) as usize).sum::<usize>())
        .sum()
}

fn part1_step(grid: &Grid, x: usize, y: usize) -> Option<bool> {
    let y = y as i64;
    let x = x as i64;
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            count += check_seat(grid, x + dx, y + dy).unwrap_or(false) as usize;
        }
    }

    let v = grid[y as usize][x as usize].unwrap();

    if v && count >= 4 {
        return Some(false);
    }

    Some(v || (!v && count == 0))
}

fn part2_step(grid: &Grid, x: usize, y: usize, max_layers: i64) -> Option<bool> {
    let y = y as i64;
    let x = x as i64;
    let mut states: [Option<bool>; 8] = [None; 8];

    for layer_idx in 1..=max_layers {
        // For each direction, save the value of the first seat seen.
        states[0] = states[0].or_else(|| check_seat(grid, x, y - layer_idx));
        states[1] = states[1].or_else(|| check_seat(grid, x + layer_idx, y - layer_idx));
        states[2] = states[2].or_else(|| check_seat(grid, x + layer_idx, y));
        states[3] = states[3].or_else(|| check_seat(grid, x + layer_idx, y + layer_idx));
        states[4] = states[4].or_else(|| check_seat(grid, x, y + layer_idx));
        states[5] = states[5].or_else(|| check_seat(grid, x - layer_idx, y + layer_idx));
        states[6] = states[6].or_else(|| check_seat(grid, x - layer_idx, y));
        states[7] = states[7].or_else(|| check_seat(grid, x - layer_idx, y - layer_idx));
    }

    let v = grid[y as usize][x as usize].unwrap();

    if v && states.iter().filter(|s| **s == Some(true)).count() >= 5 {
        return Some(false);
    }

    Some(v || (!v && !states.contains(&Some(true))))
}

fn part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();

    loop {
        // Compute the next grid state from the current grid state.
        let mut new_grid = grid.clone();

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                new_grid[y][x] = grid[y][x].and_then(|_| part1_step(&grid, x, y));
            }
        }

        if new_grid == grid {
            return count_occupied(&new_grid);
        }

        grid = new_grid;
    }
}

fn part2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    let layers = std::cmp::max(grid.len(), grid[0].len()) as i64;

    loop {
        // Compute the next grid state from the current grid state.
        let mut new_grid = grid.clone();

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                new_grid[y][x] = grid[y][x].and_then(|_| part2_step(&grid, x, y, layers));
            }
        }

        if new_grid == grid {
            return count_occupied(&new_grid);
        }

        grid = new_grid;
    }
}

pub fn main() {
    let buf = read_to_string("inputs/day_11.txt").unwrap();

    let grid = buf
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Some(true),
                    'L' => Some(false),
                    _ => None,
                })
                .collect::<Vec<Option<bool>>>()
        })
        .collect::<Grid>();

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}
