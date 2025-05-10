use std::collections::HashMap;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let instructions: Vec<Vec<String>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.split(' ').map(String::from).collect())
        .collect();

    let part1 = (0..).filter(|&init| run(&instructions, init)).next().unwrap();

    println!("{part1}");
}

fn run(instructions: &Vec<Vec<String>>, initial_a: i32) -> bool {
    let (mut a, mut b, mut c, mut d): (i32, i32, i32, i32) = (initial_a, 0, 0, 0);
    let mut pc = 0;

    let mut output_length = 0;
    let mut cache = HashMap::<(usize, i32, i32, i32, i32), usize>::new();

    while pc < instructions.len() {
        let parts = &instructions[pc];

        let state = (pc, a, b, c, d);
        if let Some(prefix_length) = cache.get(&state) {
            let repeating_length = output_length - prefix_length;
            return repeating_length > 0 && repeating_length % 2 == 0;
        } else {
            cache.insert(state, output_length);
        }

        match parts[0].as_str() {
            "cpy" => {
                *get_register_mut(&parts[2], &mut a, &mut b, &mut c, &mut d) =
                    get_register_or_value(&parts[1], a, b, c, d);
                pc += 1;
            }
            "inc" => {
                *get_register_mut(&parts[1], &mut a, &mut b, &mut c, &mut d) += 1;
                pc += 1;
            }
            "dec" => {
                *get_register_mut(&parts[1], &mut a, &mut b, &mut c, &mut d) -= 1;
                pc += 1;
            }
            "jnz" => {
                if get_register_or_value(&parts[1], a, b, c, d) != 0 {
                    pc = ((pc as i32) + parts[2].parse::<i32>().unwrap()) as usize
                } else {
                    pc += 1
                }
            }
            "out" => {
                let val = get_register_or_value(&parts[1], a, b, c, d);
                match val {
                    0 if output_length % 2 == 0 => output_length += 1,
                    1 if output_length % 2 == 1 => output_length += 1,
                    _ => return false,
                }
                pc += 1;
            }
            _ => unreachable!(),
        }
    }

    false
}

fn get_register_mut<'a>(register: &str, a: &'a mut i32, b: &'a mut i32, c: &'a mut i32, d: &'a mut i32) -> &'a mut i32 {
    match register {
        "a" => a,
        "b" => b,
        "c" => c,
        "d" => d,
        _ => unreachable!(),
    }
}

fn get_register_or_value(register_or_value: &str, a: i32, b: i32, c: i32, d: i32) -> i32 {
    match register_or_value {
        "a" => a,
        "b" => b,
        "c" => c,
        "d" => d,
        _ => register_or_value.parse().unwrap(),
    }
}
