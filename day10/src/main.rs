use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let instructions: Vec<Instruction> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|str| str.parse().ok())
        .collect();

    let mut robots: HashMap<usize, (usize, usize)> = HashMap::new();

    for robot in 0..=209 {
        fill_robot(&instructions, &mut robots, &robot);
    }

    let part1 = robots
        .iter()
        .filter_map(|(robot, (min, max))| (*min == 17 && *max == 61).then_some(*robot))
        .next()
        .unwrap();

    let part2 = [0, 1, 2]
        .iter()
        .map(|output| get_output(&instructions, &mut robots, output))
        .product::<usize>();

    println!("{part1} {part2}");
}

fn fill_robot(instructions: &Vec<Instruction>, robots: &mut HashMap<usize, (usize, usize)>, robot: &usize) {
    if robots.contains_key(&robot) {
        return;
    }

    let mut relevant_instructions = instructions
        .iter()
        .filter_map(|inst| execute_instruction(inst, instructions, robots, &Node::Bot(*robot)));

    let val_1 = relevant_instructions.next().unwrap();
    let val_2 = relevant_instructions.next().unwrap();

    robots.insert(*robot, (val_1.min(val_2), val_1.max(val_2)));
}

fn get_output(instructions: &Vec<Instruction>, robots: &mut HashMap<usize, (usize, usize)>, output: &usize) -> usize {
    instructions
        .iter()
        .filter_map(|inst| execute_instruction(inst, instructions, robots, &Node::Output(*output)))
        .next()
        .unwrap()
}

fn execute_instruction(
    instruction: &Instruction,
    instructions: &Vec<Instruction>,
    robots: &mut HashMap<usize, (usize, usize)>,
    node: &Node,
) -> Option<usize> {
    match instruction {
        Instruction::Input { value, target } if node == target => Some(*value),
        Instruction::Regular { low, high, bot } if low == node || high == node => {
            fill_robot(instructions, robots, bot);

            let (min, max) = robots.get(bot).unwrap();
            if low == node {
                Some(*min)
            } else if high == node {
                Some(*max)
            } else {
                None
            }
        }
        _ => None,
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Instruction {
    Input { value: usize, target: Node },
    Regular { bot: usize, low: Node, high: Node },
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Node {
    Bot(usize),
    Output(usize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static INPUT_PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^value (?<value>[0-9]+) goes to bot (?<bot>[0-9]+)$").unwrap());
        static REGULAR_PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^bot (?<bot>[0-9]+) gives low to (?<low>.+) and high to (?<high>.+)$").unwrap());

        if let Some(caps) = INPUT_PATTERN.captures(s) {
            Ok(Instruction::Input {
                value: caps["value"].parse().unwrap(),
                target: Node::Bot(caps["bot"].parse().unwrap()),
            })
        } else if let Some(caps) = REGULAR_PATTERN.captures(s) {
            Ok(Instruction::Regular {
                bot: caps["bot"].parse().unwrap(),
                low: caps["low"].parse().unwrap(),
                high: caps["high"].parse().unwrap(),
            })
        } else {
            Err(())
        }
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static BOT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^bot (?<id>[0-9]+)$").unwrap());
        static OUTPUT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^output (?<id>[0-9]+)$").unwrap());

        if let Some(caps) = BOT.captures(s) {
            Ok(Node::Bot(caps["id"].parse().unwrap()))
        } else if let Some(caps) = OUTPUT.captures(s) {
            Ok(Node::Output(caps["id"].parse().unwrap()))
        } else {
            Err(())
        }
    }
}
