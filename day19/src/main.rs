fn main() {
    let path = std::env::args().nth(1).unwrap();
    let n: usize = std::fs::read_to_string(path).unwrap().trim().parse().unwrap();

    let mut elves: Vec<usize> = (1..=n).collect();
    let mut keep_first = true;
    while elves.len() > 1 {
        let mut keep = std::iter::once(keep_first).chain(std::iter::once(!keep_first)).cycle();
        keep_first = if elves.len() % 2 == 0 { keep_first } else { !keep_first };
        elves.retain(|_| keep.next().unwrap());
    }
    let part1 = elves[0];

    let mut elves: Vec<bool> = std::iter::repeat_n(true, n).collect();
    let mut elves_left = n;
    let mut current = 0;
    let mut opposite = 0;
    let mut ahead = 0;
    while elves_left > 1 {
        // move opposite until the desired distance is reached
        let half_circle = elves_left / 2;
        for _ in ahead..half_circle {
            move_one_ahead(&elves, &mut opposite);
        }
        ahead = half_circle;

        // remove at that position
        elves[opposite] = false;
        elves_left -= 1;

        // move opposite to next occupied position
        // this keeps 'ahead' the same
        move_one_ahead(&elves, &mut opposite);

        // move current to next occupied position
        // this makes 'ahead' one smaller
        move_one_ahead(&elves, &mut current);
        ahead -= 1;
    }
    let part2 = elves
        .iter()
        .enumerate()
        .filter_map(|(index, alive)| alive.then_some(index + 1))
        .next()
        .unwrap();

    println!("{part1} {part2}");
}

fn move_one_ahead(elves: &Vec<bool>, pos: &mut usize) {
    *pos = (*pos + 1) % elves.len();

    while !elves[*pos] {
        *pos = (*pos + 1) % elves.len();
    }
}
