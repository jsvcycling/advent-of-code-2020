use std::fs::read_to_string;

fn rotate_around_ship(waypoint: (i32, i32), dir: i32) -> (i32, i32) {
    match dir {
        0 => waypoint,
        90 => (waypoint.1, -waypoint.0),
        180 => (-waypoint.0, -waypoint.1),
        270 => (-waypoint.1, waypoint.0),
        _ => panic!(),
    }
}

fn part1(instructions: &[(&str, i32)]) -> i32 {
    let mut dir: i32 = 90;
    let mut position: (i32, i32) = (0, 0);

    for instr in instructions {
        match instr.0 {
            "N" => position.0 += instr.1,
            "S" => position.0 -= instr.1,
            "E" => position.1 += instr.1,
            "W" => position.1 -= instr.1,
            "L" => dir = (dir - instr.1 + 360) % 360,
            "R" => dir = (dir + instr.1 + 360) % 360,
            "F" => match dir {
                0 => position.0 += instr.1,
                180 => position.0 -= instr.1,
                90 => position.1 += instr.1,
                270 => position.1 -= instr.1,
                _ => panic!("dir: {}", dir),
            },
            _ => panic!(),
        }
    }

    position.0.abs() + position.1.abs()
}

fn part2(instructions: &[(&str, i32)]) -> i32 {
    let mut waypoint: (i32, i32) = (1, 10);
    let mut ship: (i32, i32) = (0, 0);

    for instr in instructions {
        match instr.0 {
            "N" => waypoint.0 += instr.1,
            "S" => waypoint.0 -= instr.1,
            "E" => waypoint.1 += instr.1,
            "W" => waypoint.1 -= instr.1,
            "L" => waypoint = rotate_around_ship(waypoint, instr.1),
            "R" => waypoint = rotate_around_ship(waypoint, 360 - instr.1),
            "F" => {
                ship.0 += waypoint.0 * instr.1;
                ship.1 += waypoint.1 * instr.1;
            }
            _ => panic!(),
        }
    }

    ship.0.abs() + ship.1.abs()
}

pub fn main() {
    let buf = read_to_string("inputs/day_12.txt").unwrap();

    let instructions = buf
        .lines()
        .map(|l| {
            let parts = l.split_at(1);
            (parts.0, parts.1.parse::<i32>().unwrap())
        })
        .collect::<Vec<(&str, i32)>>();

    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));
}
