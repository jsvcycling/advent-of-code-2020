use std::fs::read_to_string;

fn compute(lines: &Vec<&str>, movement: (usize, usize)) -> usize {
    let mut num_trees = 0;
    let mut pos = (0, 0);

    let line_length = lines[0].len();

    while pos.1 + movement.1 < lines.len() {
        pos.0 += movement.0;
        pos.1 += movement.1;

        if pos.0 >= line_length {
            pos.0 -= line_length;
        }

        if lines[pos.1].as_bytes()[pos.0] as char == '#' {
            num_trees += 1;
        }
    }

    num_trees
}

pub fn main() {
    let buf = read_to_string("inputs/day_03.txt").unwrap();
    let lines: Vec<&str> = buf.lines().collect();

    let part1 = compute(&lines, (3, 1));

    println!("Part 1: {}", part1);

    let part_2a = compute(&lines, (1, 1));
    let part_2b = compute(&lines, (5, 1));
    let part_2c = compute(&lines, (7, 1));
    let part_2d = compute(&lines, (1, 2));

    println!("Part 2: {}", part_2a * part1 * part_2b * part_2c * part_2d);
}
