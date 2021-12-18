use std::{fs, env};

fn to_u32(bits: &[u8]) -> u32 {
    bits.iter().fold(0, |acc, &b| acc * 2 + b as u32)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Can't read input file");

    let thres = input.lines().count() as f32 / 2.0;
    println!("Line count: {}", input.lines().count());
    println!("Threshold: {}", thres);

    let mut counts = [0; 12];
    for line in input.lines() {
        for (i, char) in line.chars().enumerate() {
            counts[i] += char.to_digit(10).expect("Failed to parse character as number");
        }
    }

    println!("Counts: {:?}", counts);

    // Calculate the gamma and epsilon rates as bit arrays
    let gamma = counts.map(|count| (count as f32 / thres) as u8);
    let epsilon = gamma.map(|bit| 1 - bit);

    println!("Gamma: {:?}", gamma);
    println!("Epsilon: {:?}", epsilon);

    // Convert them to integers
    let gamma = to_u32(&gamma);
    let epsilon = to_u32(&epsilon);

    println!("{} * {} = {}", gamma, epsilon, gamma * epsilon);
}
