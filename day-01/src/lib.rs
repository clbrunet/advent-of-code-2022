pub fn part_1(input: &str) -> String {
    let max = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    max.to_string()
}

pub fn part_2(input: &str) -> String {
    let mut totals = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    totals.sort_unstable_by(|a, b| b.cmp(a));
    let sum = totals.iter().take(3).sum::<u32>();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "24000");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "45000");
    }
}
