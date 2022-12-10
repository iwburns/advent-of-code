pub fn compute(input: &str) -> String {
    let mut calorie_counts: Vec<u32> = input
        .split("\n\n")
        .map(|group_str| {
            group_str
                .lines()
                .map(|line| line.parse::<u32>().expect("invalid u32 calorie count"))
                .sum()
        })
        .collect();

    calorie_counts.sort();
    calorie_counts.reverse();

    let top_three_sum: u32 = calorie_counts.iter().take(3).sum();
    top_three_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_works() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(compute(input), "45000")
    }
}
