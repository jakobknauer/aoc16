use std::collections::{HashMap, VecDeque};

type Coord = (usize, usize);

fn main() {
    let path = std::env::args().nth(1).unwrap();

    let mut start = None;
    let mut locations = Vec::<Coord>::new();

    let map: Vec<Vec<bool>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .inspect(|&(x, c)| match c {
                    '0' => start = Some((y, x)),
                    '1'..'9' => locations.push((y, x)),
                    _ => (),
                })
                .map(|(_, c)| c != '#')
                .collect()
        })
        .collect();

    let start = start.unwrap();
    locations.insert(0, start);

    let distances: Vec<Vec<u32>> = locations
        .iter()
        .map(|loc| find_distances_to_locations(loc, &locations, &map))
        .collect();

    let mut to_visit: Vec<bool> = vec![true; locations.len()];
    to_visit[0] = false;

    let part1 = find_shortest_route(&0, &mut to_visit, &distances);
    let part2 = find_shortest_roundtrip(&0, &mut to_visit, &distances);

    println!("{part1} {part2}");
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn find_distances_to_locations(start: &Coord, locations: &Vec<Coord>, map: &Vec<Vec<bool>>) -> Vec<u32> {
    let mut open = VecDeque::<Coord>::new();
    open.push_back(*start);

    let mut distances = HashMap::<Coord, u32>::new();
    distances.insert(*start, 0);

    while !open.is_empty() {
        let (y, x) = open.pop_front().unwrap();
        let current_distance = distances[&(y, x)];

        for (dy, dx) in DIRECTIONS {
            let (ny, nx) = ((y as i32 + dy) as usize, (x as i32 + dx) as usize);
            if !distances.contains_key(&(ny, nx)) && map[ny][nx] {
                distances.insert((ny, nx), current_distance + 1);
                open.push_back((ny, nx));
            }
        }
    }

    locations.iter().map(|loc| distances[loc]).collect()
}

fn find_shortest_route(start_index: &usize, to_visit: &mut Vec<bool>, distances: &Vec<Vec<u32>>) -> u32 {
    let mut shortest: Option<u32> = None;

    let indices_to_visit: Vec<_> = to_visit
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.then_some(i))
        .collect();

    for next in indices_to_visit {
        to_visit[next] = false;
        let distance = distances[*start_index][next] + find_shortest_route(&next, to_visit, &distances);
        to_visit[next] = true;
        shortest = shortest.map(|v| v.min(distance)).or(Some(distance));
    }

    shortest.unwrap_or(0)
}

fn find_shortest_roundtrip(start_index: &usize, to_visit: &mut Vec<bool>, distances: &Vec<Vec<u32>>) -> u32 {
    let mut shortest: Option<u32> = None;

    let indices_to_visit: Vec<_> = to_visit
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.then_some(i))
        .collect();

    for next in indices_to_visit {
        to_visit[next] = false;
        let distance = distances[*start_index][next] + find_shortest_roundtrip(&next, to_visit, &distances);
        to_visit[next] = true;
        shortest = shortest.map(|v| v.min(distance)).or(Some(distance));
    }

    shortest.unwrap_or(distances[*start_index][0])
}
