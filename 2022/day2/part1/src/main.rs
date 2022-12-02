use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn from_char(char: &char) -> Hand {
        let val = (*char as u32 - 'A' as u32) % 23;

        match val {
            0 => Hand::Rock,
            1 => Hand::Paper,
            2 => Hand::Scissors,
            _ => panic!("Invalid hand value {}", char),
        }
    }

    fn play(&self, theirs: &Hand) -> i32 {
        let result = (*self as i32 - *theirs as i32 + 1).rem_euclid(3);

        let result_points = result * 3;
        let choice_points = *self as i32 + 1;

        result_points + choice_points
    }
}

fn main() {
    let mut score = 0;

    for line_res in read_lines("../input.txt") {
        match line_res {
            Ok(line) => {
                let hands = line
                    .split_whitespace()
                    .map(|c| c.chars().next().unwrap())
                    .map(|c| Hand::from_char(&c))
                    .collect::<Vec<Hand>>();

                score += hands[1].play(&hands[0]);
            }
            Err(error) => panic!("Problem reading line: {:?}", error),
        };
    }

    println!("The score is: {}", score);
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
