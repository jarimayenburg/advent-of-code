use std::collections::HashSet;

use aoc_2022::read_lines;

type Item = char;

type Compartment = Vec<Item>;

type Rucksack = (Compartment, Compartment);

type Group = Vec<Rucksack>;

trait Badge {
    fn badge(&self) -> Item;
}

impl Badge for Group {
    fn badge(&self) -> Item {
        let mut shared_items: HashSet<&Item> = HashSet::from_iter(self[0].all_items());

        for rucksack in self {
            let items = HashSet::from_iter(rucksack.all_items());

            shared_items = shared_items
                .intersection(&items)
                .map(|a| a.to_owned())
                .collect::<HashSet<&Item>>();
        }

        **shared_items.iter().next().unwrap()
    }
}

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

trait Items {
    fn common_items(&self) -> Vec<&Item>;
    fn all_items(&self) -> Vec<&Item>;
}

impl Items for Rucksack {
    /// Find the items that are in both compartments
    fn common_items(&self) -> Vec<&Item> {
        let comp1_set: HashSet<&Item> = HashSet::from_iter(self.0.iter());
        let comp2_set: HashSet<&Item> = HashSet::from_iter(self.1.iter());

        comp1_set
            .intersection(&comp2_set)
            .map(|i| i.to_owned())
            .collect()
    }

    fn all_items(&self) -> Vec<&Item> {
        self.0.iter().chain(self.1.iter()).collect()
    }
}

fn part1(rucksacks: &Vec<Rucksack>) -> u32 {
    rucksacks
        .iter()
        .flat_map(|rucksack| rucksack.common_items())
        .map(|item| item.priority())
        .sum()
}

fn part2(rucksacks: &Vec<Rucksack>) -> u32 {
    rucksacks
        .chunks(3)
        .map(|s| s.to_owned())
        .map(|group: Group| group.badge())
        .map(|badge| badge.priority())
        .sum()
}

fn main() {
    let rucksacks = read_rucksacks();

    println!("Part 1: {}", part1(&rucksacks));
    println!("Part 2: {}", part2(&rucksacks));
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
