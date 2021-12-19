use std::{env, fs};

fn build_cards(inputs: &Vec<&str>) -> Vec<Vec<Vec<u32>>> {
    inputs
        .iter()
        .map(|card| {
            card.split("\n")
                .map(|row| {
                    row.split(" ")
                        .filter(|c| !c.is_empty())
                        .map(|c| c.trim().parse::<u32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn column(matrix: &Vec<Vec<u32>>, idx: usize) -> Vec<u32> {
    matrix.iter().map(|row| *row.get(idx).unwrap()).collect()
}

fn has_won(card: &Vec<Vec<u32>>, calls: &Vec<u32>) -> bool {
    // Check if any of the rows is fully called
    card.iter().any(|row| row.iter().all(|val| calls.contains(val))) ||

    // Check if any of the columns is fully called
    (0..card.get(0).unwrap().len()).map(|i| column(card, i)).any(|col| col.iter().all(|val| calls.contains(val)))
}

fn calc_score(card: &Vec<Vec<u32>>, calls: &Vec<u32>, last_called: u32) -> u32 {
    let card_sum: u32 = card
        .iter()
        .map(|row| {
            row.iter().fold(0, |accum, val| {
                if calls.contains(val) {
                    accum
                } else {
                    accum + val
                }
            })
        })
        .sum();

    card_sum * last_called
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Can't read input file");

    let mut raw_inputs: Vec<&str> = input.split("\n\n").collect();

    let calls: Vec<u32> = raw_inputs
        .remove(0)
        .split(",")
        .map(|c| c.parse::<u32>().unwrap())
        .collect();

    let cards = build_cards(&raw_inputs);

    let mut called: Vec<u32> = Vec::new();

    for call in &calls {
        called.push(*call);

        for card in &cards {
            if has_won(&card, &called) {
                println!(
                    "Card has won with score: {}",
                    calc_score(&card, &called, *call)
                );
                return;
            }
        }
    }
}
