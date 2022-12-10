use super::*;

pub fn compute(input: &str) -> String {
    let scores = character_priorities();

    let total_score = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let duplicate = left
                .chars()
                .find(|c| right.contains(*c))
                .expect("couldn't find duplicate item");
            scores
                .get(&duplicate)
                .expect(&format!("no score for duplicate: {}", duplicate))
        })
        .sum::<usize>();

    total_score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(compute(input), "157")
    }
}
