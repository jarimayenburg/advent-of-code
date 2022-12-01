use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut cal_counts = vec![0; 1000000];
    let mut i = 0;

    for line_res in read_lines("../input.txt") {
        match line_res {
            Ok(line) => {
                if line.is_empty() {
                    i += 1
                } else {
                    cal_counts[i] += line.parse::<i32>().unwrap();
                }
            }
            Err(error) => panic!("Problem reading line: {:?}", error),
        };
    }

    cal_counts.sort();

    let top_three: Vec<&i32> = cal_counts.iter().rev().take(3).collect::<Vec<&i32>>();

    println!("Top three calories: {:?}", top_three);
    println!("Total of top three: {}", top_three.into_iter().sum::<i32>())
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
