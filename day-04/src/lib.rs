use std::ops::RangeInclusive;

fn parse_range(range: &str) -> RangeInclusive<u32> {
    let (start, end) = range
        .split_once('-').unwrap();
    start.parse().unwrap()..=end.parse().unwrap()
}

fn parse_ranges(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let (first, second) = line.split_once(',').unwrap();
    (parse_range(first), parse_range(second))
}

pub fn part_1(input: &str) -> String {
    let count = input
        .lines()
        .filter(|&line| {
            let (first, second) = parse_ranges(line);
            (first.contains(second.start()) && first.contains(second.end()))
                || (second.contains(first.start()) && second.contains(first.end()))
        })
        .count();
    count.to_string()
}

pub fn part_2(input: &str) -> String {
    let count = input
        .lines()
        .filter(|&line| {
            let (mut first, second) = parse_ranges(line);
            first.any(|n| second.contains(&n))
        })
        .count();
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "2");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "4");
    }
}
