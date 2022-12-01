use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut max = 0;
    let mut count = 0;

    for line_res in read_lines("../input.txt") {
        match line_res {
            Ok(line) => {
                if line.is_empty() {
                    if count > max {
                        max = count;
                    }

                    count = 0
                } else {
                    count += line.parse::<i32>().unwrap();
                }
            }
            Err(error) => panic!("Problem reading line: {:?}", error),
        };
    }

    println!("Max calories is {}", max);
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
