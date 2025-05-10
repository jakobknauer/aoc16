fn main() {
    let path = std::env::args().nth(1).unwrap();
    let instructions: Vec<Vec<String>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.split(' ').map(String::from).collect())
        .collect();

    // For checking what the code is actually doing:
    // for n in 7.. {
    //     let result = run(&instructions, n);
    //     let result2 = 7031 + (1..=n).product::<i32>();
    //     println!("{n}\t{result}\t{result2}");
    // }

    let part1 = run(&instructions, 7);
    let part2 = 7031 + (1..=12).product::<i32>();

    println!("{part1} {part2}");
}

fn run(instructions: &Vec<Vec<String>>, initial_a: i32) -> i32 {
    let mut instructions = instructions.clone();

    let (mut a, mut b, mut c, mut d): (i32, i32, i32, i32) = (initial_a, 0, 0, 0);
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
                    let offset = get_register_or_value(&parts[2], a, b, c, d);
                    pc = ((pc as i32) + offset) as usize;
                } else {
                    pc += 1
                }
            }
            "tgl" => {
                let offset = get_register_or_value(&parts[1], a, b, c, d);
                let target_index = ((pc as i32) + offset) as usize;

                let Some(target_line) = instructions.get_mut(target_index) else {
                    pc += 1;
                    continue;
                };

                target_line[0] = match (target_line[0].as_str(), target_line.len()) {
                    ("inc", 2) => String::from("dec"),
                    (_, 2) => String::from("inc"),
                    ("jnz", 3) => String::from("cpy"),
                    (_, 3) => String::from("jnz"),
                    _ => unreachable!(),
                };

                pc += 1;
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
        _ => panic!(),
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
