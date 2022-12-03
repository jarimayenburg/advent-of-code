use std::path::Path;

use aoc_2022::read_lines;

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
    fn from_int(int: i32) -> Hand {
        match int.rem_euclid(3) {
            0 => Hand::Rock,
            1 => Hand::Paper,
            2 => Hand::Scissors,
            _ => panic!("Invalid hand value {}", int),
        }
    }

    fn points(&self, outcome: &Outcome) -> i32 {
        let result_points = *outcome as i32 * 3;
        let choice_points = *self as i32 + 1;

        result_points + choice_points
    }

    fn play(&self, theirs: &Hand) -> i32 {
        let outcome = Outcome::from_int(*self as i32 - *theirs as i32 + 1);

        self.points(&outcome)
    }

    /// Returns which hand should be played against this hand to achieve the desired outcome
    fn achieve(&self, outcome: &Outcome) -> Hand {
        Hand::from_int(*self as i32 + (*outcome as i32 - 1))
    }
}

impl FromChar<Hand> for Hand {
    fn from_char(char: &char) -> Hand {
        Hand::from_int((*char as i32 - 'A' as i32) % 23)
    }
}

fn part1() -> i32 {
    read_games("inputs/day2.txt")
        .iter()
        .map(|hand_chars| {
            (
                Hand::from_char(&hand_chars.0),
                Hand::from_char(&hand_chars.1),
            )
        })
        .map(|(theirs, ours)| ours.play(&theirs))
        .sum()
}

fn part2() -> i32 {
    read_games("inputs/day2.txt")
        .iter()
        .map(|chars| (Hand::from_char(&chars.0), Outcome::from_char(&chars.1)))
        .map(|(theirs, outcome)| {
            let ours = theirs.achieve(&outcome);

            (theirs, ours)
        })
        .map(|(theirs, ours)| ours.play(&theirs))
        .sum()
}

fn main() {
    println!("Part 1: The score is {}", part1());
    println!("Part 2: The score is {}", part2());
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
