use regex::Regex;

fn main() {
    let rect = Regex::new(r"^rect (?<A>[0-9]+)x(?<B>[0-9]+)$").unwrap();
    let rotate_row = Regex::new(r"^rotate row y=(?<A>[0-9]+) by (?<B>[0-9]+)$").unwrap();
    let rotate_column = Regex::new(r"^rotate column x=(?<A>[0-9]+) by (?<B>[0-9]+)$").unwrap();

    let path = std::env::args().nth(1).unwrap();

    let operations: Vec<_> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|op| parse_operation(op, &rect, &rotate_row, &rotate_column))
        .collect();

    let mut screen = [[false; 50]; 6];

    for op in operations {
        match op {
            Operation::Rect(a, b) => {
                for i in 0..a {
                    for j in 0..b {
                        screen[j][i] = true;
                    }
                }
            }
            Operation::RotateRow(a, b) => {
                let new_row: Vec<_> = (0..50).map(|i| screen[a][(i + 50 - b) % 50]).collect();
                for i in 0..50 {
                    screen[a][i] = new_row[i];
                }
            }
            Operation::RotateColumn(a, b) => {
                let new_column: Vec<_> = (0..6).map(|i| screen[(i + 6 - b) % 6][a]).collect();
                for i in 0..6 {
                    screen[i][a] = new_column[i];
                }
            }
        };
    }

    let part1: i32 = screen
        .iter()
        .map(|row| row.iter().map(|b| *b as i32).sum::<i32>())
        .sum();

    println!("{part1}");

    for row in screen {
        let row: String = row.iter().map(|b| if *b { '#' } else { ' ' }).collect();
        println!("{row}");
    }
}

#[derive(Debug)]
enum Operation {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

fn parse_operation(op: &str, rect: &Regex, rotate_row: &Regex, rotate_column: &Regex) -> Operation {
    if let Some(caps) = rect.captures(op) {
        Operation::Rect(caps["A"].parse().unwrap(), caps["B"].parse().unwrap())
    } else if let Some(caps) = rotate_row.captures(op) {
        Operation::RotateRow(caps["A"].parse().unwrap(), caps["B"].parse().unwrap())
    } else if let Some(caps) = rotate_column.captures(op) {
        Operation::RotateColumn(caps["A"].parse().unwrap(), caps["B"].parse().unwrap())
    } else {
        unreachable!()
    }
}
