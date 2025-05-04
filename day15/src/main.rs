fn main() {
    let path = std::env::args().nth(1).unwrap();

    let mut equations: Vec<(usize, usize)> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(' ').map(str::parse).map(Result::unwrap);
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();

    let part1 = (0..)
        .filter(|t_0| {
            equations
                .iter()
                .enumerate()
                .all(|(disk_index, &(n_positions, p_0))| (t_0 + disk_index + p_0 + 1) % n_positions == 0)
        })
        .next()
        .unwrap();

    equations.push((11, 0));
    let part2 = (0..)
        .filter(|t_0| {
            equations
                .iter()
                .enumerate()
                .all(|(disk_index, &(n_positions, p_0))| (t_0 + disk_index + p_0 + 1) % n_positions == 0)
        })
        .next()
        .unwrap();

    println!("{part1} {part2}");
}
