use std::collections::HashMap;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let contents = std::fs::read_to_string(path).unwrap();

    let rooms: Vec<_> = contents.lines().map(parse_room).collect();

    let part1: i32 = rooms
        .iter()
        .filter(|room| is_valid_room(&room))
        .map(|room| room.id)
        .sum();

    let secret_room = rooms
        .iter()
        .map(|room| (room, decipher_name(room)))
        .filter(|(_, deciphered)| deciphered.contains("northpole"))
        .next()
        .unwrap();

    // println!("{}", secret_room.1);
    let part2 = secret_room.0.id;

    println!("{part1} {part2}");
}

struct Room {
    name: String,
    id: i32,
    checksum: String,
}

fn parse_room(description: &str) -> Room {
    let first_digit_index = description
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_ascii_digit())
        .next()
        .unwrap()
        .0;

    let first_bracket = description
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '[')
        .next()
        .unwrap()
        .0;

    Room {
        name: description[..first_digit_index - 1].into(),
        id: description[first_digit_index..first_bracket].parse().unwrap(),
        checksum: description[first_bracket + 1..description.len() - 1].into(),
    }
}

fn is_valid_room(room: &Room) -> bool {
    let letters = room.name.chars().filter(|c| c.is_alphabetic());

    let mut letter_frequencies = HashMap::<char, usize>::new();
    for letter in letters {
        if !letter_frequencies.contains_key(&letter) {
            letter_frequencies.insert(letter, 0);
        }

        *letter_frequencies.get_mut(&letter).unwrap() += 1;
    }

    let mut letter_frequencies: Vec<_> = letter_frequencies.iter().collect();
    letter_frequencies.sort_by(|(l1, c1), (l2, c2)| c1.cmp(c2).reverse().then(l1.cmp(l2)));

    let checksum: String = letter_frequencies.iter().take(5).map(|(l, _)| *l).collect();

    checksum == room.checksum
}

fn decipher_name(room: &Room) -> String {
    room.name
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| (c as i32) - ('a' as i32))
        .map(|n| (n + room.id) % 26)
        .map(|n| ((n as u8) + ('a' as u8)) as char)
        .collect()
}
