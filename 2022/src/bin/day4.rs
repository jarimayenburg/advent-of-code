use aoc_2022::read_lines;

struct SectionRange {
    start: u32,
    end: u32,
}

impl SectionRange {
    fn overlaps(&self, other: &SectionRange) -> bool {
        !(self.start > other.end || self.end < other.start)
    }

    fn contains(&self, other: &SectionRange) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

fn part1(sections: &Vec<(SectionRange, SectionRange)>) -> usize {
    sections
        .iter()
        .filter(|(sec1, sec2)| sec1.contains(sec2) || sec2.contains(sec1))
        .count()
}

fn part2(sections: &Vec<(SectionRange, SectionRange)>) -> usize {
    sections
        .iter()
        .filter(|(sec1, sec2)| sec1.overlaps(sec2))
        .count()
}

fn main() {
    let sections = read_section_pairs();

    println!("Part 1: {}", part1(&sections));
    println!("Part 2: {}", part2(&sections));
}

fn read_section_pairs() -> Vec<(SectionRange, SectionRange)> {
    read_lines("inputs/day4.txt")
        .map(|line| line.unwrap())
        .map(|line| {
            let (range1, range2) = line.split_once(",").unwrap();

            (range1.to_owned(), range2.to_owned())
        })
        .map(|(range1, range2)| {
            let ((start1, end1), (start2, end2)) = (
                range1.split_once("-").unwrap(),
                range2.split_once("-").unwrap(),
            );

            (
                (start1.to_owned(), end1.to_owned()),
                (start2.to_owned(), end2.to_owned()),
            )
        })
        .map(|((start1, end1), (start2, end2))| {
            (
                SectionRange {
                    start: start1.parse::<u32>().unwrap(),
                    end: end1.parse::<u32>().unwrap(),
                },
                SectionRange {
                    start: start2.parse::<u32>().unwrap(),
                    end: end2.parse::<u32>().unwrap(),
                },
            )
        })
        .collect()
}
