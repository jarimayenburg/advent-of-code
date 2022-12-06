use std::collections::HashSet;

use aoc_2022::read_lines;

fn search(chars: &Vec<char>, window: usize) -> usize {
    chars
        .windows(window)
        .enumerate()
        .find(|(_, window)| HashSet::<&char>::from_iter(window.iter()).len() == window.len())
        .unwrap()
        .0
        + window
}

fn main() {
    let chars = read_lines("inputs/day6.txt")
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect();

    println!("Part 1: {}", search(&chars, 4));
    println!("Part 2: {}", search(&chars, 14));
}
