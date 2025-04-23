use std::collections::HashMap;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let contents = std::fs::read_to_string(path).unwrap();

    let lines: Vec<_> = contents.lines().collect();

    let length = lines[0].len();

    let part1: String = (0..length)
        .map(|i| {
            let column = lines.iter().map(|line| line.chars().nth(i).unwrap());
            get_most_frequent(column)
        })
        .collect();

    let part2: String = (0..length)
        .map(|i| {
            let column = lines.iter().map(|line| line.chars().nth(i).unwrap());
            get_least_frequent(column)
        })
        .collect();

    println!("{part1} {part2}");
}

fn get_most_frequent(column: impl IntoIterator<Item = char>) -> char {
    let mut counts = HashMap::new();
    for c in column {
        *counts.entry(c).or_insert(0) += 1;
    }

    counts
        .iter()
        .max_by(|(_, count_1), (_, count_2)| count_1.cmp(count_2))
        .unwrap()
        .0
        .clone()
}

fn get_least_frequent(column: impl IntoIterator<Item = char>) -> char {
    let mut counts = HashMap::new();
    for c in column {
        *counts.entry(c).or_insert(0) += 1;
    }

    counts
        .iter()
        .min_by(|(_, count_1), (_, count_2)| count_1.cmp(count_2))
        .unwrap()
        .0
        .clone()
}
