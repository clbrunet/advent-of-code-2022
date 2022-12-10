pub fn part_1(input: &str) -> String {
    let mut x = 1;
    let mut cycle = 1;
    let mut signals_sum = 0;
    for line in input.lines() {
        if (cycle - 20) % 40 == 0 {
            signals_sum += cycle * x;
        }
        let mut words = line.split_whitespace();
        match words.next().unwrap() {
            "noop" => cycle += 1,
            "addx" => {
                if (cycle + 1 - 20) % 40 == 0 {
                    signals_sum += (cycle + 1) * x;
                }
                cycle += 2;
                x += words.next().unwrap().parse::<i32>().unwrap()
            },
            _ => panic!(),

        }
    }
    signals_sum.to_string()
}

pub fn part_2(input: &str) -> String {
    let mut x = 1;
    let mut cycle = 1;
    const CRT_WIDTH: usize = 40;
    const CRT_HEIGHT: usize = 6;
    let mut crt = vec!['.'; CRT_WIDTH * CRT_HEIGHT];
    for line in input.lines() {
        if x - 1 <= (cycle - 1) % 40  && (cycle - 1) % 40 <= x + 1 {
            crt[cycle as usize - 1] = '#';
        }
        let mut words = line.split_whitespace();
        match words.next().unwrap() {
            "noop" => cycle += 1,
            "addx" => {
                if x - 1 <= cycle % 40  && cycle % 40 <= x + 1 {
                    crt[cycle as usize] = '#';
                }
                cycle += 2;
                x += words.next().unwrap().parse::<i32>().unwrap()
            },
            _ => panic!(),

        }
    }
    for i in (CRT_WIDTH..(CRT_WIDTH * CRT_HEIGHT)).step_by(CRT_WIDTH).rev() {
        crt.insert(i, '\n');
    }
    crt.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "13140");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....");
    }
}
