use itertools::Itertools;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let initial = std::fs::read_to_string(path).unwrap().trim().to_owned();

    const DISK_LENGTH_1: usize = 272;
    const DISK_LENGTH_2: usize = 35651584;

    let part1 = checksum(&generate(&initial, DISK_LENGTH_1));
    let part2 = checksum(&generate(&initial, DISK_LENGTH_2));

    println!("{part1} {part2}");
}

fn generate(initial: &str, target_size: usize) -> String {
    let mut data: String = initial.to_owned();

    while data.len() < target_size {
        let inverse: String = data
            .chars()
            .map(|c| match c {
                '0' => '1',
                '1' => '0',
                _ => unreachable!(),
            })
            .collect();
        data.push('0');
        data.extend(inverse.chars().rev());
    }

    data[..target_size].to_owned()
}

fn checksum(data: &str) -> String {
    let mut checksum: String = data.to_owned();

    while checksum.len() % 2 == 0 {
        checksum = checksum
            .chars()
            .chunks(2)
            .into_iter()
            .map(|chunk| {
                let chunk: Vec<char> = chunk.collect();
                if chunk[0] == chunk[1] { '1' } else { '0' }
            })
            .collect();
    }

    checksum
}
