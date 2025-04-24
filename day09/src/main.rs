fn main() -> Result<(), ()> {
    let path = std::env::args().nth(1).ok_or(())?;
    let contents = std::fs::read_to_string(path).or(Err(()))?;
    let contents = contents.trim();

    let part1 = version_one(contents).ok_or(())?;
    let part2 = version_two(contents).ok_or(())?;

    println!("{part1} {part2}");

    Ok(())
}

fn version_one(mut compressed: &str) -> Option<usize> {
    let mut decompressed = 0;

    while compressed.len() > 0 {
        let next = compressed.chars().next()?;
        if next != '(' {
            decompressed += 1;
            compressed = &compressed[1..];
        } else {
            let (to_repeat, repetitions, rest) = parse_marker(compressed)?;
            decompressed += to_repeat.len() * repetitions;
            compressed = rest;
        }
    }

    Some(decompressed)
}

fn version_two(mut compressed: &str) -> Option<usize> {
    let mut decompressed = 0;

    while compressed.len() > 0 {
        let next = compressed.chars().next()?;
        if next != '(' {
            decompressed += 1;
            compressed = &compressed[1..];
        } else {
            let (to_repeat, repetitions, rest) = parse_marker(compressed)?;
            decompressed += version_two(to_repeat)? * repetitions;
            compressed = rest;
        }
    }

    Some(decompressed)
}

fn parse_marker(compressed: &str) -> Option<(&str, usize, &str)> {
    assert!(compressed.chars().next()? == '(');
    let rest = &compressed[1..];

    let (n_chars, rest) = rest.split_once('x')?;
    let n_chars: usize = n_chars.parse().ok()?;

    let (repetitions, rest) = rest.split_once(')')?;
    let repetitions: usize = repetitions.parse().ok()?;

    Some((&rest[..n_chars], repetitions, &rest[n_chars..]))
}
