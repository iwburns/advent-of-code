use std::str::FromStr;

mod part_1;
mod part_2;

#[derive(Copy, Clone, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn beats(&self) -> Self {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn loses(&self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("no input file found");
    println!("part 1: {}", part_1::compute(&input));
    println!("part 2: {}", part_2::compute(&input));
}
