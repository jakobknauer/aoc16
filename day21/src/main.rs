use regex::Regex;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let instructions: Vec<_> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(parse_instruction)
        .collect();

    let mut password: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    scramble(&mut password, &instructions);
    let part1: String = password.iter().collect();

    let mut password: [char; 8] = ['f', 'b', 'g', 'd', 'c', 'e', 'a', 'h'];
    unscramble(&mut password, &instructions);
    let part2: String = password.iter().collect();

    println!("{part1} {part2}");
}

fn scramble<'a, const N: usize>(password: &mut [char; N], instructions: impl IntoIterator<Item = &'a Instruction>) {
    for instruction in instructions {
        match *instruction {
            Instruction::SwapPositions(a, b) => password.swap(a, b),
            Instruction::SwapLetters(a, b) => {
                let a = password.iter().position(|&c| c == a).unwrap();
                let b = password.iter().position(|&c| c == b).unwrap();
                password.swap(a, b);
            }
            Instruction::Reverse(a, b) => password[a..=b].reverse(),
            Instruction::RotateLeft(steps) => password.rotate_left(steps),
            Instruction::RotateRight(steps) => password.rotate_right(steps),
            Instruction::Move { from, to } if from < to => password[from..=to].rotate_left(1),
            Instruction::Move { from, to } if from > to => password[to..=from].rotate_right(1),
            Instruction::RotateBasedOn(letter) => {
                let pos = password.iter().position(|&c| c == letter).unwrap();
                password.rotate_right(1);
                password.rotate_right(pos);
                if pos >= 4 {
                    password.rotate_right(1);
                };
            }
            _ => unreachable!(),
        }
    }
}

fn unscramble<'a, const N: usize, I>(password: &mut [char; N], instructions: I)
where
    I: IntoIterator<Item = &'a Instruction>,
    I::IntoIter: DoubleEndedIterator,
{
    for instruction in instructions.into_iter().rev() {
        match *instruction {
            Instruction::SwapPositions(a, b) => password.swap(a, b),
            Instruction::SwapLetters(a, b) => {
                let a = password.iter().position(|&c| c == a).unwrap();
                let b = password.iter().position(|&c| c == b).unwrap();
                password.swap(a, b);
            }
            Instruction::Reverse(a, b) => password[a..=b].reverse(),
            Instruction::RotateLeft(steps) => password.rotate_right(steps),
            Instruction::RotateRight(steps) => password.rotate_left(steps),
            Instruction::Move { from, to } if from < to => password[from..=to].rotate_right(1),
            Instruction::Move { from, to } if from > to => password[to..=from].rotate_left(1),
            Instruction::RotateBasedOn(letter) => {
                let pos_after = password.iter().position(|&c| c == letter).unwrap();
                let pos_before = match pos_after {
                    0 => 7,
                    1 => 0,
                    2 => 4,
                    3 => 1,
                    4 => 5,
                    5 => 2,
                    6 => 6,
                    7 => 3,
                    _ => unreachable!(),
                };
                password.rotate_left((pos_after - pos_before) % N);
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    SwapPositions(usize, usize),
    SwapLetters(char, char),
    Reverse(usize, usize),
    RotateLeft(usize),
    RotateRight(usize),
    Move { from: usize, to: usize },
    RotateBasedOn(char),
}

fn parse_instruction(instruction: &str) -> Instruction {
    let swap_positions = Regex::new(r"swap position (?<A>[0-9]+) with position (?<B>[0-9]+)").unwrap();
    let swap_letters = Regex::new(r"swap letter (?<A>[a-z]) with letter (?<B>[a-z])").unwrap();
    let reverse = Regex::new(r"reverse positions (?<A>[0-9]) through (?<B>[0-9])").unwrap();
    let rotate_left = Regex::new(r"rotate left (?<steps>[0-9]+) steps?").unwrap();
    let rotate_right = Regex::new(r"rotate right (?<steps>[0-9]+) steps?").unwrap();
    let move_ = Regex::new(r"move position (?<from>[0-9]+) to position (?<to>[0-9]+)").unwrap();
    let rotate_based_on = Regex::new(r"rotate based on position of letter (?<A>[a-z])").unwrap();

    if let Some(caps) = swap_positions.captures(instruction) {
        Instruction::SwapPositions(caps["A"].parse().unwrap(), caps["B"].parse().unwrap())
    } else if let Some(caps) = swap_letters.captures(instruction) {
        Instruction::SwapLetters(caps["A"].parse().unwrap(), caps["B"].parse().unwrap())
    } else if let Some(caps) = reverse.captures(instruction) {
        Instruction::Reverse(caps["A"].parse().unwrap(), caps["B"].parse().unwrap())
    } else if let Some(caps) = rotate_left.captures(instruction) {
        Instruction::RotateLeft(caps["steps"].parse().unwrap())
    } else if let Some(caps) = rotate_right.captures(instruction) {
        Instruction::RotateRight(caps["steps"].parse().unwrap())
    } else if let Some(caps) = move_.captures(instruction) {
        Instruction::Move {
            from: caps["from"].parse().unwrap(),
            to: caps["to"].parse().unwrap(),
        }
    } else if let Some(caps) = rotate_based_on.captures(instruction) {
        Instruction::RotateBasedOn(caps["A"].parse().unwrap())
    } else {
        unreachable!("Invalid instruction: {instruction}")
    }
}
