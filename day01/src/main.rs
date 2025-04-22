use num::complex::Complex;
use std::collections::HashSet;
use std::iter::repeat_n;

type Coord = Complex<i32>;

struct Instruction {
    rotation: Coord,
    count: usize,
}

fn main() {
    let path = std::env::args().nth(1).unwrap();

    let instructions: Vec<Instruction> = std::fs::read_to_string(path)
        .unwrap()
        .split(",")
        .map(str::trim)
        .map(parse_instruction)
        .collect();

    let step_directions = instructions
        .into_iter()
        .scan(Coord::ONE, |orientation, Instruction { rotation, count }| {
            *orientation *= rotation;
            Some(repeat_n(orientation.clone(), count))
        })
        .flatten();

    let route = step_directions.scan(Coord::ZERO, |pos, movement| {
        *pos += movement;
        Some(*pos)
    });

    let (last_position, first_duplication, _) = route.fold(
        (Coord::ZERO, None, HashSet::from([Coord::ZERO])),
        |(_, mut first_duplication, mut visited), position| {
            if first_duplication.is_none() && visited.contains(&position) {
                first_duplication = Some(position)
            };
            visited.insert(position);
            (position, first_duplication, visited)
        },
    );

    let part1 = last_position.l1_norm();
    let part2 = first_duplication.unwrap().l1_norm();

    println!("{part1} {part2}");
}

fn parse_instruction(instruction: &str) -> Instruction {
    let (direction, count) = instruction.split_at(1);

    let rotation = match direction {
        "R" => Complex::I,
        "L" => -Complex::I,
        _ => panic!(),
    };

    let count = count.parse().unwrap();

    Instruction { rotation, count }
}
