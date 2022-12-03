pub fn part_1(input: &str) {
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
    println!("Part 1 : {}", max);
}

pub fn part_2(input: &str) {
    let mut totals = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        }).collect::<Vec<u32>>();
    totals.sort_unstable_by(|a, b| b.cmp(a));
    let sum = totals.iter().take(3).sum::<u32>();
    println!("Part 2 : {}", sum);
}
