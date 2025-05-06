fn main() {
    let path = std::env::args().nth(1).unwrap();
    let mut excludes: Vec<(u64, u64)> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let (from, to) = line.trim().split_once("-").unwrap();
            (from.parse().unwrap(), to.parse().unwrap())
        })
        .collect();
    excludes.sort_by_key(|&(from, _)| from);

    let mut current_lower_bound = 0;
    let mut part1 = None;
    let mut part2 = 0;

    for (from, to) in excludes {
        if current_lower_bound < from {
            part1.get_or_insert(current_lower_bound);
            part2 += from - current_lower_bound;
        }
        current_lower_bound = current_lower_bound.max(to + 1);
    }

    let part1 = part1.unwrap();

    println!("{part1} {part2}");
}
