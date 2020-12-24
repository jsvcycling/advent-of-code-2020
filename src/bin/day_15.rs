use std::collections::HashMap;

fn compute(target: usize, input: &[usize]) -> usize {
    // Store the previous value.
    let mut last = 0;

    // Store the locations of all previous ocurrances of each value.
    let mut history: HashMap<usize, Vec<usize>> = HashMap::new();

    for (idx, val) in input.iter().enumerate() {
        last = *val;
        history
            .entry(last)
            .and_modify(|v| v.push(idx))
            .or_insert(vec![idx]);
    }

    for i in input.len()..target {
        let prev = history.get(&last).unwrap();

        if prev.len() == 1 {
            last = 0;
        } else {
            last = prev[prev.len() - 1] - prev[prev.len() - 2];
        }

        history
            .entry(last)
            .and_modify(|v| v.push(i))
            .or_insert(vec![i]);
    }

    last
}

pub fn main() {
    let input = "0,20,7,16,1,18,15"
        .split(',')
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", compute(2020, &input));
    println!("Part 2: {}", compute(30_000_000, &input));
}
