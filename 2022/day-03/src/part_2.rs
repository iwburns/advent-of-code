use super::*;
use itertools::Itertools;

pub fn compute(input: &str) -> String {
    let scores = character_priorities();

    let total_score = input
        .lines()
        .tuples()
        .map(|(line1, line2, line3)| {
            let triplicate = line1
                .chars()
                .find(|c| line2.contains(*c) && line3.contains(*c))
                .expect("couldn't find triplicate item");
            scores
                .get(&triplicate)
                .expect(&format!("no score for triplicate: {}", triplicate))
        })
        .sum::<usize>();

    total_score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_works() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(compute(input), "70")
    }
}
