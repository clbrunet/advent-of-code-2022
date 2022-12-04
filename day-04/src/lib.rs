use std::ops::RangeInclusive;

pub fn part_1(input: &str) -> String {
    let count = input
        .lines()
        .filter(|&line| {
            let ranges = line
                .split(',')
                .map(|str_range| {
                    let mut iter = str_range.split('-');
                    let start = iter.next().unwrap().parse::<u32>().unwrap();
                    let end = iter.next().unwrap().parse::<u32>().unwrap();
                    start..=end
                })
                .collect::<Vec<RangeInclusive<u32>>>();
            let first = ranges[0].to_owned();
            let mut second = ranges[1].to_owned();
            first.clone().all(|n| second.contains(&n)) || second.all(|n| first.contains(&n))
        })
        .count();
    count.to_string()
}

pub fn part_2(input: &str) -> String {
    let count = input
        .lines()
        .filter(|&line| {
            let ranges = line
                .split(',')
                .map(|str_range| {
                    let mut iter = str_range.split('-');
                    let start = iter.next().unwrap().parse::<u32>().unwrap();
                    let end = iter.next().unwrap().parse::<u32>().unwrap();
                    start..=end
                })
                .collect::<Vec<RangeInclusive<u32>>>();
            let mut first = ranges[0].to_owned();
            let second = ranges[1].to_owned();
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
