use std::{env, fs, fmt::Debug};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32
}

#[derive(Debug)]
struct Vent {
    start: Point,
    end: Point,
}

impl Vent {
    fn from_input(input: &str) -> Vec<Vent> {
        input.lines()
            .map(|line| {
                let words: Vec<&str> = line.split(" ").collect();
                let (start_word, end_word ): (Vec<&str>, Vec<&str>) = (words[0].split(",").collect(), words[2].split(",").collect());

                Vent {
                    start: Point {
                        x: start_word[0].parse().unwrap(),
                        y: start_word[1].parse().unwrap()
                    },
                    end: Point {
                        x: end_word[0].parse().unwrap(),
                        y: end_word[1].parse().unwrap()
                    }
                }
            })
            .collect()
    }

    fn line(&self) -> Vec<Point> {
        if self.start.x == self.end.x {
            let range = if self.end.y > self.start.y {
                self.start.y..self.end.y + 1
            }
            else {
                self.end.y..self.start.y + 1
            };

            range.map(|y| Point { x: self.start.x, y }).collect()
        }
        else if self.start.y == self.end.y {
            let range = if self.end.x > self.start.x {
                self.start.x..self.end.x + 1
            }
            else {
                self.end.x..self.start.x + 1
            };

            range.map(|x| Point { x, y: self.start.y }).collect()
        }
        else {
            Vec::new()
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Can't read input file");

    let mut vents = Vent::from_input(&input);

    // Filter out diagonal vents
    vents.retain(|vent| vent.start.x == vent.end.x || vent.start.y == vent.end.y);

    let lines: Vec<Vec<Point>> = vents.iter().map(|vent| vent.line()).collect();

    let max_x: usize = vents.iter().flat_map(|vent| [vent.start.x, vent.end.x]).max().unwrap() as usize;
    let max_y: usize = vents.iter().flat_map(|vent| [vent.start.y, vent.end.y]).max().unwrap() as usize;

    let mut grid: Vec<Vec<u32>> = vec![vec![0; max_y + 1]; max_x + 1];

    for line in lines {
        for point in line {
            grid[point.x as usize][point.y as usize] += 1;
        }
    }

    let points: usize = grid.into_iter().flatten().filter(|c| *c > 1).count();

    println!("Total points: {}", points);
}
