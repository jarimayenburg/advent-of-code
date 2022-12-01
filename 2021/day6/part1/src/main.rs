use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Can't read input file");

    let mut fish: Vec<i32> = input.split(",").map(|f| f.parse().unwrap()).collect();

    for _day in 0..80 {
        let new_fish_count = fish.iter().filter(|&f| *f == 0).count();

        // Decrement all fish values
        fish.iter_mut().for_each(|f| *f -= 1);

        // Add new fishes
        fish.append(&mut vec![8; new_fish_count]);

        // Replace all -1 values with 6s
        fish.iter_mut().filter(|f| f.is_negative()).for_each(|f| *f = 6);
    }

    println!("After 80 days, there are {} fish", fish.len());
}
