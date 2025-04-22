use std::io::Write;

fn main() {
    let input = std::env::args().nth(1).unwrap();

    let part1: String = (0..)
        .map(|i| format!("{input}{i}"))
        .map(md5::compute)
        .map(|hash| format!("{hash:x}"))
        .filter(|hash| hash.starts_with("00000"))
        .map(|hash| hash.chars().nth(5).unwrap())
        .inspect(|char| {
            print!("{char}");
            _ = std::io::stdout().flush();
        })
        .take(8)
        .collect();

    println!("");

    let mut part2_characters = (0..)
        .map(|i| format!("{input}{i}"))
        .map(md5::compute)
        .map(|hash| format!("{hash:x}"))
        .filter(|hash| hash.starts_with("00000"))
        .filter(|hash| hash.chars().nth(5).unwrap().is_ascii_digit())
        .filter_map(|hash| {
            let index: usize = (hash.chars().nth(5).unwrap() as usize) - ('0' as usize);
            if index < 8 {
                let char = hash.chars().nth(6).unwrap();
                Some((index, char))
            } else {
                None
            }
        });

    let mut part2 = ['_'; 8];
    let mut defined_chars = 0;

    while defined_chars < 8 {
        let (index, character) = part2_characters.next().unwrap();

        if part2[index] == '_' {
            part2[index] = character;
            defined_chars += 1;
            println!("{}", part2.iter().collect::<String>());
        }
    }

    let part2: String = part2.iter().collect();

    println!("{part1} {part2}");
}
