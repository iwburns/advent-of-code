use std::collections::HashMap;

mod part_1;
mod part_2;

fn character_priorities() -> HashMap<char, usize> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("no input file found");
    println!("part 1: {}", part_1::compute(&input));
    println!("part 2: {}", part_2::compute(&input));
}
