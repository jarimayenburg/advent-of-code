use std::{env, fs};

fn to_u32(bits: &[u32]) -> u32 {
    bits.iter().fold(0, |acc, &b| acc * 2 + b as u32)
}

fn column(matrix: &Vec<Vec<u32>>, idx: usize) -> Vec<u32> {
    matrix.iter().map(|row| *row.get(idx).unwrap()).collect()
}

#[derive(Debug)]
enum Rating {
    Oxygen,
    CO2
}

impl Rating {
    fn filter_criteria(&self, idx: usize, bits: &[u32], vals: &mut Vec<Vec<u32>>) -> () {
        let thres = bits.len() as f32 / 2.0;
        println!("Thres: {}", thres);

        match self {
            Rating::Oxygen => {
                println!("Sum: {}", bits.iter().sum::<u32>());
                let most_common = (bits.iter().sum::<u32>() as f32 / thres) as u32;
                println!("Most common: {}", most_common);

                vals.retain(|row| *row.get(idx).unwrap() == most_common);
            },
            Rating::CO2 => {
                println!("Sum: {}", bits.iter().sum::<u32>());
                let least_common = 1- (bits.iter().sum::<u32>() as f32 / thres) as u32;
                println!("Least common: {}", least_common);

                vals.retain(|row| *row.get(idx).unwrap() == least_common);
            }
        }
    }

    fn calculate(&self, vals: &Vec<Vec<u32>>) -> Option<u32> {
        let mut vals = vals.clone();
        let width = vals.get(0).unwrap().len();

        for i in 0..width {
            let col = column(&vals, i);

            println!("{:?} values before filter #{} has size: {}", self, i, vals.len());

            self.filter_criteria(i, &col, &mut vals);

            println!("{:?} values after filter #{} has size: {}", self, i, vals.len());

            if vals.len() == 1 {
                return Some(to_u32(vals.get(0).unwrap()))
            }
        }

        None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Can't read input file");

    let vals: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|char| char.to_digit(2).unwrap() as u32).collect())
        .collect();

    let oxygen = Rating::Oxygen.calculate(&vals).unwrap();
    let co2 = Rating::CO2.calculate(&vals).unwrap();
    let lifesupport = oxygen * co2;

    println!("Oxygen: {}, C02: {}, Life support: {}", oxygen, co2, lifesupport);
}
