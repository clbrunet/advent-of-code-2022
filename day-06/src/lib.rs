fn contains_duplicate(chs: &[char]) -> bool {
    for (j, ch) in chs.iter().enumerate() {
        if chs[(j + 1)..].contains(ch) {
            return true;
        }
    }
    false
}

fn get_marker_end(signal: &str, len: usize) -> String {
    let chs = signal.trim().chars().collect::<Vec<_>>();
    for i in len..chs.len() {
        if !contains_duplicate(&chs[(i - len)..i]) {
            return i.to_string();
        }
    }
    "".into()
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
