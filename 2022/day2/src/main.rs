use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

trait FromChar<T> {
    fn from_char(char: &char) -> T;
}

#[derive(Debug)]
enum Outcome {
    Lose,
    Tie,
    Win,
}

impl Outcome {
    fn from_int(result: i32) -> Outcome {
        match result.rem_euclid(3) {
            0 => Outcome::Lose,
            1 => Outcome::Tie,
            2 => Outcome::Win,
            _ => panic!("Invalid result value {}", result),
        }
    }
}

impl FromChar<Outcome> for Outcome {
    fn from_char(char: &char) -> Outcome {
        match char {
            'X' => Outcome::Lose,
            'Y' => Outcome::Tie,
            'Z' => Outcome::Win,
            _ => panic!("Invalid value: {}", char),
        }
    }
}

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn points(&self, outcome: &Outcome) -> i32 {
        let result_points = *outcome as i32 * 3;
        let choice_points = *self as i32 + 1;

        result_points + choice_points
    }

    fn play(&self, theirs: &Hand) -> i32 {
        let outcome = Outcome::from_int(*self as i32 - *theirs as i32 + 1);

        self.points(&outcome)
    }
}

impl FromChar<Hand> for Hand {
    fn from_char(char: &char) -> Hand {
        let val = (*char as u32 - 'A' as u32) % 23;

        match val {
            0 => Hand::Rock,
            1 => Hand::Paper,
            2 => Hand::Scissors,
            _ => panic!("Invalid hand value {}", char),
        }
    }
}

fn part1() -> i32 {
    read_games("input.txt")
        .iter()
        .map(|hand_strs| (Hand::from_char(&hand_strs.0), Hand::from_char(&hand_strs.1)))
        .map(|(theirs, ours)| ours.play(&theirs))
        .sum()
}

fn main() {
    println!("Part 1: The score is {}", part1());
}

fn read_games<P>(filename: P) -> Vec<(char, char)>
where
    P: AsRef<Path>,
{
    read_lines(filename)
        .map(|line| line.unwrap())
        .map(|line| {
            line.split_once(" ")
                .map(|(c1, c2)| (c1.chars().next().unwrap(), c2.chars().next().unwrap()))
                .unwrap()
        })
        .collect()
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename);

    match file {
        Ok(file) => io::BufReader::new(file).lines(),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}
