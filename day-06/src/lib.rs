use std::collections::HashSet;

fn get_marker_end(signal: &str, len: usize) -> String {
    let chs = signal.trim().chars().collect::<Vec<_>>();
    let i = chs
        .windows(len)
        .enumerate()
        .find(|&(_, chs)| len == chs.iter().collect::<HashSet<_>>().len())
        .unwrap()
        .0;
    (i + len).to_string()
}

pub fn part_1(input: &str) -> String {
    get_marker_end(input, 4)
}

pub fn part_2(input: &str) -> String {
    get_marker_end(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb
";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "7");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "19");
    }
}
