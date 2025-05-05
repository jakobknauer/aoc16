fn main() {
    let path = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(path).unwrap();

    let mut current: Vec<bool> = input
        .trim()
        .chars()
        .map(|c| match c {
            '.' => true,
            '^' => false,
            _ => unreachable!(),
        })
        .collect();

    let mut safe_count = current.iter().filter(|&&x| x).count();

    for _ in 1..40 {
        current = (0..current.len() as i32)
            .map(|i| {
                is_safe([i - 1, i, i + 1].map(|j| {
                    if j < 0 {
                        true
                    } else {
                        current.get(j as usize).copied().unwrap_or(true)
                    }
                }))
            })
            .collect();
        safe_count += current.iter().filter(|&&x| x).count();
    }
    let part1 = safe_count;

    for _ in 40..400000 {
        current = (0..current.len() as i32)
            .map(|i| {
                is_safe([i - 1, i, i + 1].map(|j| {
                    if j < 0 {
                        true
                    } else {
                        current.get(j as usize).copied().unwrap_or(true)
                    }
                }))
            })
            .collect();
        safe_count += current.iter().filter(|&&x| x).count();
    }
    let part2 = safe_count;

    println!("{part1} {part2}");
}

fn is_safe([l, c, r]: [bool; 3]) -> bool {
    match (l, c, r) {
        (false, false, true) | (true, false, false) | (false, true, true) | (true, true, false) => false,
        _ => true,
    }
}
