use aoc_2022::read_lines;

fn part1() {
    let mut max = 0;
    let mut count = 0;

    for line_res in read_lines("inputs/day1.txt") {
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

fn part2() {
    let mut cal_counts = vec![0; 1000000];
    let mut i = 0;

    for line_res in read_lines("inputs/day1.txt") {
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

fn main() {
    part1();
    part2();
}
