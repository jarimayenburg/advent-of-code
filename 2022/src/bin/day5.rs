use aoc_2022::read_lines;

type Crate = char;

type Stack = Vec<Crate>;

type Instruction = (usize, usize, usize);

fn part1(mut stacks: Vec<Stack>, instructions: &Vec<Instruction>) -> String {
    for (amount, from, to) in instructions {
        for _ in 0..*amount {
            let c = stacks[*from].pop().unwrap();

            stacks[*to].push(c);
        }
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn part2(mut stacks: Vec<Stack>, instructions: &Vec<Instruction>) -> String {
    for (amount, from, to) in instructions {
        let from_len = stacks[*from].len();

        let mut crates = stacks[*from].drain(from_len - amount..).collect();

        stacks[*to].append(&mut crates);
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn main() {
    let (stacks, instructions) = read_input();

    println!("Part 1: {}", part1(stacks.clone(), &instructions));
    println!("Part 2: {}", part2(stacks, &instructions));
}

fn read_input() -> (Vec<Stack>, Vec<Instruction>) {
    let mut lines = read_lines("inputs/day5.txt").filter_map(|x| x.ok());

    let mut stack_lines = vec![];
    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        stack_lines.push(line);
    }

    let crate_idx_line = stack_lines.pop().unwrap();

    // Pad crate lines on the right
    stack_lines = stack_lines
        .into_iter()
        .map(|line| format!("{:0width$}", line, width = crate_idx_line.len() + 1))
        .collect();

    // Collect the remaining lines as instructions
    let instruction_lines = lines.collect::<Vec<String>>();

    (
        parse_stacks(stack_lines),
        parse_instructions(instruction_lines),
    )
}

fn parse_stacks(stack_lines: Vec<String>) -> Vec<Stack> {
    let mut stack_rows = stack_lines
        .iter()
        .map(|line| line.chars().skip(1).step_by(4).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Flip the stacks
    stack_rows.reverse();

    transpose(stack_rows)
        .into_iter()
        .map(|stack| {
            stack
                .into_iter()
                .take_while(|c| !c.is_whitespace())
                .collect()
        })
        .collect()
}

fn parse_instructions(instruction_lines: Vec<String>) -> Vec<Instruction> {
    instruction_lines
        .iter()
        .map(|line| line.split_whitespace())
        .map(|words| {
            words
                .filter_map(|word| word.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .map(|nums| (nums[0], nums[1] - 1, nums[2] - 1))
        .collect()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
