fn main() {
    let path = std::env::args().nth(1).unwrap();

    let instructions: Vec<_> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(str::to_owned)
        .collect();

    let part1 = get_code::<3, 3, 1, 1>(&BOARD_1, &instructions);
    let part2 = get_code::<5, 5, 0, 2>(&BOARD_2, &instructions);

    println!("{part1} {part2}");
}

const BOARD_1: [[Option<char>; 3]; 3] = [
    [Some('1'), Some('2'), Some('3')],
    [Some('4'), Some('5'), Some('6')],
    [Some('7'), Some('8'), Some('9')],
];

const BOARD_2: [[Option<char>; 5]; 5] = [
    [None, None, Some('1'), None, None],
    [None, Some('2'), Some('3'), Some('4'), None],
    [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
    [None, Some('A'), Some('B'), Some('C'), None],
    [None, None, Some('D'), None, None],
];

fn get_code<'a, const WIDTH: usize, const HEIGHT: usize, const START_X: usize, const START_Y: usize>(
    board: &[[Option<char>; WIDTH]; HEIGHT],
    instructions: impl IntoIterator<Item = impl AsRef<str>>,
) -> String {
    let (mut x, mut y) = (START_X, START_Y);
    let mut word = String::new();

    for instruction in instructions {
        (x, y) = instruction.as_ref().chars().fold((x, y), |(x, y), direction| {
            let (dx, dy) = char_to_direction(direction);
            let (new_x, new_y) = (x as i32 + dx, y as i32 + dy);
            if is_valid_board_position(&board, new_x, new_y) {
                (new_x as usize, new_y as usize)
            } else {
                (x, y)
            }
        });
        word.push(board[x][y].unwrap());
    }

    word
}

fn char_to_direction(c: char) -> (i32, i32) {
    match c {
        'U' => (-1, 0),
        'D' => (1, 0),
        'L' => (0, -1),
        'R' => (0, 1),
        _ => panic!(),
    }
}

fn is_valid_board_position<const WIDTH: usize, const HEIGHT: usize>(
    board: &[[Option<char>; WIDTH]; HEIGHT],
    x: i32,
    y: i32,
) -> bool {
    0 <= x && (x as usize) < WIDTH && 0 <= y && (y as usize) < HEIGHT && board[x as usize][y as usize].is_some()
}
