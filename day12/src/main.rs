fn main() {
    let path = std::env::args().nth(1).unwrap();
    let instructions: Vec<Vec<String>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.split(' ').map(String::from).collect())
        .collect();

    let part1 = run(&instructions, 0);
    let part2 = run(&instructions, 1);

    println!("{part1} {part2}");
}

fn run(instructions: &Vec<Vec<String>>, initial_c: i32) -> i32 {
    let (mut a, mut b, mut c, mut d): (i32, i32, i32, i32) = (0, 0, initial_c, 0);
    let mut pc = 0;

    while pc < instructions.len() {
        let parts = &instructions[pc];

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
            _ => unreachable!(),
        }
    }

    a
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
