use super::*;
use itertools::Itertools;

pub fn compute(input: &str) -> String {
    let count = input
        .lines()
        .filter(|&line| {
            let mut ranges = line.split(",").map(|s| RangeInclusive::from_str(s));

            let range_a = ranges.next().expect("missing first range");
            let range_b = ranges.next().expect("missing second range");

            range_a.overlaps_with(&range_b)
        })
        .count();

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_works() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(compute(input), "4")
    }
}
