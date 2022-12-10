mod part_1;
mod part_2;

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("no input file found");
    println!("part 1: {}", part_1::compute(&input));
    println!("part 2: {}", part_2::compute(&input));
}
