use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

// We could use the same Graph structure for both parts but that'd be
// inefficent for Part 1.
type Part1Graph = HashMap<String, HashSet<String>>;
type Part2Graph = HashMap<String, HashMap<String, usize>>;

// For Part 1, the outer HashMap is keyed by the "inner bags". For example,
// the line "A contains 2 B, 3 C." would be stored as
// { "B": ["A"], "C": ["A"] }. This allows us to efficiently traverse the graph
// from the inside out (from the most inner bag to the most outer bag). This
// way, we can find all that bags that contain the input bag and recursively
// work our way further out. We use a set as the inner data structure because
// we don't care about the *number* of bags, we only care about the fact that
// they are there.
fn build_graph_part1(buf: &str) -> Part1Graph {
    let mut g: Part1Graph = HashMap::new();

    buf.lines().for_each(|l| {
        let parts = l.split("bags contain").collect::<Vec<_>>();
        let outer = parts[0].trim().to_string();

        parts[1].trim().split(',').for_each(|p| {
            let subparts = p.trim().splitn(2, ' ').collect::<Vec<_>>();

            // Even though we don't care about the number of bags, we still need
            // to try and parse a count to distinguish between a number and "no
            // bags".
            if subparts[0].parse::<usize>().is_ok() {
                let name = subparts[1]
                    .trim_end_matches(" bag")
                    .trim_end_matches(" bags")
                    .trim_end_matches(" bag.")
                    .trim_end_matches(" bags.");

                g.entry(name.to_string())
                    .and_modify(|v| {
                        v.insert(outer.clone());
                    })
                    .or_insert({
                        let mut h = HashSet::new();
                        h.insert(outer.clone());
                        h
                    });
            }
        });
    });

    g
}

// Here we do the opposite from part 1, building the graph from the outside in.
// So the same line "A contains 2 B, 3 C." will now be stored as
// { "A": { "B": 2, "C": 3 } }. Now we can efficiently traverse the graph from
// the outside in.
fn build_graph_part2(buf: &str) -> Part2Graph {
    let mut g: Part2Graph = HashMap::new();

    buf.lines().for_each(|l| {
        let parts = l.split("bags contain").collect::<Vec<_>>();
        let outer = parts[0].trim().to_string();

        parts[1].trim().split(',').for_each(|p| {
            let subparts = p.trim().splitn(2, ' ').collect::<Vec<_>>();

            if let Ok(count) = subparts[0].parse::<usize>() {
                let name = subparts[1]
                    .trim_end_matches(" bag")
                    .trim_end_matches(" bags")
                    .trim_end_matches(" bag.")
                    .trim_end_matches(" bags.");

                g.entry(outer.to_string())
                    .and_modify(|v| {
                        v.insert(name.to_string(), count);
                    })
                    .or_insert({
                        let mut h = HashMap::new();
                        h.insert(name.to_string(), count);
                        h
                    });
            }
        });
    });

    g
}

// Traverse the part1 graph, building a set of all the bags encountered.
fn execute_part1(bags: &mut HashSet<String>, graph: &Part1Graph, target: &str) {
    if let Some(g) = graph.get(target) {
        g.iter().for_each(|k| {
            bags.insert(k.clone());
            execute_part1(bags, graph, k)
        });
    }
}

// Traverse the part2 graph, counting the number of bags encountered.
fn execute_part2(graph: &Part2Graph, target: &str) -> usize {
    if let Some(g) = graph.get(target) {
        // We need to count the current bag as well as all its inner bags.
        return g.iter().map(|(k, v)| v + v * execute_part2(graph, k)).sum();
    }

    0
}

pub fn main() {
    let target = "shiny gold".to_string();
    let buf = read_to_string("inputs/day_07.txt").unwrap();

    let part1_graph = build_graph_part1(&buf);
    let part2_graph = build_graph_part2(&buf);

    let mut bags: HashSet<String> = HashSet::new();
    execute_part1(&mut bags, &part1_graph, &target);

    let result = execute_part2(&part2_graph, &target);

    println!("Part 1: {}", bags.len());
    println!("Part 2: {}", result);
}
