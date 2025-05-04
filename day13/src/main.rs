use std::collections::{HashMap, VecDeque};

const START: Coord = (1, 1);
const TARGET: Coord = (31, 39);

type Coord = (i32, i32);

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let salt: i32 = std::fs::read_to_string(path).unwrap().trim().parse().unwrap();

    let mut open: VecDeque<Coord> = VecDeque::new();
    open.push_back(START);

    let mut distances: HashMap<Coord, i32> = HashMap::new();
    distances.insert(START, 0);

    while !open.is_empty() {
        let current = open.pop_front().unwrap();
        let current_distance = distances[&current];

        if distances.contains_key(&TARGET) && current_distance >= 50 {
            continue;
        }

        for n in iter_neighbors(&current, salt) {
            if !distances.contains_key(&n) {
                open.push_back(n);
                distances.insert(n, current_distance + 1);
            }
        }
    }

    let part1 = distances[&TARGET];
    let part2 = distances.into_iter().filter(|&(_, distance)| distance <= 50).count();

    println!("{part1} {part2}");
}

fn iter_neighbors(&(x, y): &Coord, salt: i32) -> impl IntoIterator<Item = Coord> {
    [(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)]
        .into_iter()
        .filter(|&(x, y)| x >= 0 && y >= 0)
        .filter(move |&(x, y)| {
            let n = x * x + 3 * x + 2 * x * y + y + y * y + salt;
            let s = format!("{n:b}");
            let one_bits = s.chars().filter(|&c| c == '1').count();
            one_bits % 2 == 0
        })
}
