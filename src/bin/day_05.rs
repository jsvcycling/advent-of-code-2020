use std::fs::read_to_string;

pub fn main() {
    let buf = read_to_string("inputs/day_05.txt").unwrap();

    let mut ids: Vec<i32> = buf
        .lines()
        .map(|line| {
            let mut rows = (0, 127);
            let mut seats = (0, 7);

            line.chars().for_each(|c| {
                match c {
                    'F' => {
                        rows.1 = (rows.1 + rows.0) / 2;
                    }
                    'B' => {
                        rows.0 = (rows.1 + rows.0 + 1) / 2;
                    }
                    'L' => {
                        seats.1 = (seats.1 + seats.0) / 2;
                    }
                    'R' => {
                        seats.0 = (seats.1 + seats.0 + 1) / 2;
                    }
                    _ => panic!("Received: {:?}", c),
                };
            });

            rows.0 * 8 + seats.0
        })
        .collect();

    ids.sort_unstable();

    println!("Part 1: {}", ids.last().unwrap());

    // We can do this more efficiently using a BSP again but we'll just do it
    // linearly. This will run in O(n) which isn't terrible for this case (a BSP
    // would run in O(log n) by comparison).
    for (idx, val) in ids.iter().enumerate() {
        if ids[idx + 1] != val + 1 {
            println!("Part 2: {}", val + 1);
            break;
        }
    }
}
