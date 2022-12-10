pub fn compute(input: &str) -> String {
    let max_calorie_count: u32 = input
        .split("\n\n")
        .map(|group_str| {
            group_str
                .lines()
                .map(|line| line.parse::<u32>().expect("invalid u32 calorie count"))
                .sum()
        })
        .max()
        .unwrap();

    max_calorie_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
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
        assert_eq!(compute(input), "24000")
    }
}
