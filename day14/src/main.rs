use std::collections::VecDeque;

use itertools::Itertools;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let salt = std::fs::read_to_string(path).unwrap();
    let salt = salt.trim();

    let hashes = (0..).map(|i| {
        let text = format!("{salt}{i}");
        let hash = md5::compute(text);
        format!("{hash:x}")
    });
    let part1 = generate_keys(hashes);

    let hashes2 = (0..).map(|i| {
        let text = format!("{salt}{i}");
        let mut hash = text;
        for _ in 0..2017 {
            hash = format!("{:x}", md5::compute(hash));
        }
        hash
    });
    let part2 = generate_keys(hashes2);

    println!("{part1} {part2}");
}

fn generate_keys(mut hashes: impl Iterator<Item = String>) -> usize {
    let mut window: VecDeque<String> = (0..=1000).map(|_| hashes.next().unwrap()).collect();
    let mut index = 0;
    let mut keys = 0;

    while keys < 64 {
        let key = window.pop_front().unwrap();

        if let Some(c) = first_triplet(&key) {
            if window.iter().any(|key| has_quintuple(&key, c)) {
                keys += 1;
            }
        }

        index += 1;
        window.push_back(hashes.next().unwrap());
    }

    index - 1
}

fn first_triplet(hay: &str) -> Option<char> {
    hay.chars()
        .tuple_windows()
        .filter_map(|(a, b, c)| (a == b && b == c).then_some(a))
        .next()
}

fn has_quintuple(hay: &str, character: char) -> bool {
    hay.chars()
        .tuple_windows()
        .any(|(a, b, c, d, e)| a == character && b == character && c == character && d == character && e == character)
}
