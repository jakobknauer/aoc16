use std::{collections::HashSet, iter};

use itertools::Itertools;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let contents = std::fs::read_to_string(path).unwrap();

    let part1 = contents.lines().filter(|address| has_tls(address)).count();
    let part2 = contents.lines().filter(|address| has_ssl(address)).count();

    println!("{part1} {part2}");
}

fn has_tls(address: &str) -> bool {
    has_abba(address) && !get_hypernet_sequences(address).any(has_abba)
}

fn has_ssl(address: &str) -> bool {
    let abas: HashSet<_> = get_supernet_sequences(address).flat_map(get_abas).collect();
    get_hypernet_sequences(address)
        .flat_map(get_abas)
        .any(|(b, a)| abas.contains(&(a, b)))
}

fn get_hypernet_sequences(address: &str) -> impl Iterator<Item = &str> {
    let opening_indices = address.chars().enumerate().filter_map(|(i, c)| (c == '[').then_some(i));
    let closing_indices = address.chars().enumerate().filter_map(|(i, c)| (c == ']').then_some(i));
    opening_indices.zip(closing_indices).map(|(i, j)| &address[i + 1..j])
}

fn get_supernet_sequences(address: &str) -> impl Iterator<Item = &str> {
    let opening_brackets = address
        .chars()
        .enumerate()
        .filter_map(|(i, c)| (c == '[').then_some(i as i32));
    let closing_brackets = address
        .chars()
        .enumerate()
        .filter_map(|(i, c)| (c == ']').then_some(i as i32));

    let opening_indices = iter::once(-1).chain(closing_brackets);
    let closing_indices = opening_brackets.chain(iter::once(address.len() as i32));

    opening_indices
        .zip(closing_indices)
        .map(|(i, j)| &address[(i + 1) as usize..j as usize])
}

fn has_abba(address: &str) -> bool {
    address
        .chars()
        .tuple_windows()
        .any(|(a, b, c, d)| a == d && b == c && a != b)
}

fn get_abas(address: &str) -> impl Iterator<Item = (char, char)> {
    address
        .chars()
        .tuple_windows()
        .filter_map(|(a, b, c)| (a == c && a != b).then_some((a, b)))
}
