fn main() {
    let path = std::env::args().nth(1).unwrap();

    let triangles: Vec<Vec<i32>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .collect::<Vec<_>>()
        })
        .collect();

    let part1 = triangles
        .iter()
        .filter(|v| {
            let (s0, s1, s2) = (v[0], v[1], v[2]);
            (s0 + s1) > s2 && (s0 + s2) > s1 && (s1 + s2) > s0
        })
        .count();

    let part2 = triangles
        .chunks(3)
        .flat_map(|v| (0..3).map(|i| [v[0][i], v[1][i], v[2][i]]))
        .filter(|v| {
            let (v0, v1, v2) = (v[0], v[1], v[2]);
            (v0 + v1) > v2 && (v0 + v2) > v1 && (v1 + v2) > v0
        })
        .count();

    println!("{part1} {part2}");
}
