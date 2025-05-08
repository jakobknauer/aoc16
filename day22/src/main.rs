use itertools::Itertools;
use regex::Regex;

fn main() {
    let coordinates_regex = Regex::new(r"node-x(?<x>[0-9]+)-y(?<y>[0-9]+)").unwrap();

    let path = std::env::args().nth(1).unwrap();
    let nodes: Vec<Node> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .skip(2)
        .map(|line| parse_node(line, &coordinates_regex))
        .collect();

    let part1 = nodes
        .iter()
        .cartesian_product(&nodes)
        .filter(|(a, b)| a != b && a.used > 0 && a.used < b.avail)
        .count();

    // "Manual" solution
    print_grid(&nodes); // the empty node is (17,22)
    let part2 = 17      // move gap to the left
        + 2             // move gap up 2
        + 34            // move gap over almost to right border
        + 20            // move gap to the top; gap is now left of the goal
        + ( 1           // move goal one to the left; gap is now right of the goal;
          + 4 )         // then move gap in a U-shape to the left of goal
          * 34          // and repeat that 34 times, so that gap is at (0,0), goal is at (1,0)
        + 1             // move goal to (0,0)
    ;

    // The "correct" solution would be A* with a suitable heuristic,
    // perhaps "distance of goal to (0,0) + distance of gap to goal".
    // The states could be either in the shape of 'nodes', i.e. Vec<Node>;
    // or, in order to improve performance, an "abstracted" version of that,
    // where a node is either empty (gap), (almost) full, "normal", or "the goal".
    // I assume that even the non-abstracted version is feasible.

    println!("{part1} {part2}");
}

fn print_grid(nodes: &Vec<Node>) {
    for y in 0..=24 {
        for x in 0..=35 {
            let node = nodes.get(x * 25 + y).unwrap();
            let c = if node.x == 0 && node.y == 0 {
                '*'
            } else if node.x == 35 && node.y == 0 {
                'G'
            } else if node.used > 400 {
                '#'
            } else if node.avail > 80 {
                '_'
            } else {
                '.'
            };
            print!("{c}");
        }
        println!();
    }
}

#[derive(PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    used: usize,
    avail: usize,
}

fn parse_node(line: &str, coordinates: &Regex) -> Node {
    let mut parts = line.split_whitespace();

    let (x, y) = if let Some(caps) = coordinates.captures(parts.next().unwrap()) {
        (caps["x"].parse().unwrap(), caps["y"].parse().unwrap())
    } else {
        panic!()
    };

    parts.next();
    let used = parts.next().unwrap().trim_end_matches('T').parse().unwrap();
    let avail = parts.next().unwrap().trim_end_matches('T').parse().unwrap();

    Node { x, y, used, avail }
}
