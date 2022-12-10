use std::ops::RangeInclusive;

mod part_1;
mod part_2;

trait RangeExt {
    fn from_str(s: &str) -> Self;
    fn fully_contains(&self, other: &Self) -> bool;
    fn overlaps_with(&self, other: &Self) -> bool;
}

impl RangeExt for RangeInclusive<u32> {
    fn from_str(s: &str) -> Self {
        let mut parts = s
            .split("-")
            .map(|s| s.parse::<u32>().expect("invalid range bound"));

        let start = parts.next().expect("missing start bound");
        let end = parts.next().expect("missing end bound");

        RangeInclusive::new(start, end)
    }

    fn fully_contains(&self, other: &RangeInclusive<u32>) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    fn overlaps_with(&self, other: &RangeInclusive<u32>) -> bool {
        self.fully_contains(other)
            || other.fully_contains(self)
            // self     |---|
            // other      |---|
            || (self.start() <= other.start() && self.end() >= other.start())
            // self       |---|
            // other    |---|
            || (self.start() <= other.end() && self.end() >= other.end())
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("no input file found");
    println!("part 1: {}", part_1::compute(&input));
    println!("part 2: {}", part_2::compute(&input));
}
