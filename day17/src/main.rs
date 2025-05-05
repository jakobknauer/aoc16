use std::collections::VecDeque;

type Coord = (i32, i32);
type Path = String;
type State = (Coord, Path);

const DIRECTIONS: [(char, i32, i32); 4] = [('U', 0, -1), ('D', 0, 1), ('L', -1, 0), ('R', 1, 0)];

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let passcode = std::fs::read_to_string(path).unwrap();
    let passcode = passcode.trim();

    let mut open: VecDeque<State> = VecDeque::new();
    open.push_back(((0, 0), "".into()));

    let mut paths_to_vault = Vec::<Path>::new();

    while !open.is_empty() {
        let ((x, y), path) = open.pop_front().unwrap();
        let hash = format!("{:x}", md5::compute(format!("{passcode}{path}")));

        for (&(dir_symbol, dx, dy), hash_char) in DIRECTIONS.iter().zip(hash.chars()) {
            if hash_char == 'a' || hash_char.is_ascii_digit() {
                continue;
            }

            let (x, y) = (x + dx, y + dy);

            if x < 0 || x > 3 || y < 0 || y > 3 {
                continue;
            }

            let path = format!("{path}{dir_symbol}");

            if (x, y) == (3, 3) {
                paths_to_vault.push(path);
            } else {
                open.push_back(((x, y), path));
            }
        }
    }

    let part1 = paths_to_vault.iter().min_by_key(|path| path.len()).unwrap();
    let part2 = paths_to_vault.iter().max_by_key(|path| path.len()).unwrap().len();

    println!("{part1} {part2}");
}
