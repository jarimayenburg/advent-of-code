use std::collections::HashSet;

use aoc_2022::read_lines;

type Item = char;

trait Prioritize {
    fn priority(&self) -> u32;
}

impl Prioritize for Item {
    fn priority(&self) -> u32 {
        if self.is_ascii_uppercase() {
            *self as u32 - 38
        } else {
            *self as u32 - 96
        }
    }
}

type Compartment = Vec<Item>;

type Rucksack = (Compartment, Compartment);

trait CommonItem {
    fn common_items(&self) -> Vec<&Item>;
}

impl CommonItem for Rucksack {
    /// Find the items that are in both compartments
    fn common_items(&self) -> Vec<&Item> {
        let comp1_set: HashSet<&Item> = HashSet::from_iter(self.0.iter());
        let comp2_set: HashSet<&Item> = HashSet::from_iter(self.1.iter());

        comp1_set
            .intersection(&comp2_set)
            .map(|i| i.to_owned())
            .collect()
    }
}

fn part1(rucksacks: Vec<Rucksack>) -> u32 {
    rucksacks
        .iter()
        .flat_map(|rucksack| rucksack.common_items())
        .map(|item| item.priority())
        .sum()
}

fn main() {
    let rucksacks = read_rucksacks();

    println!("Part1: {}", part1(rucksacks));
}

fn read_rucksacks() -> Vec<Rucksack> {
    read_lines("inputs/day3.txt")
        .map(|line| line.unwrap())
        .map(|line| {
            let (comp1, comp2) = line.split_at(line.len() / 2);

            (
                comp1.to_owned().chars().collect(),
                comp2.to_owned().chars().collect(),
            )
        })
        .collect::<Vec<Rucksack>>()
}
