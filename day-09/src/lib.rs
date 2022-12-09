use std::{
    collections::{HashMap, HashSet},
    ops::{AddAssign, Sub},
};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

pub fn part_1(input: &str) -> String {
    let map = HashMap::from([
        ("U", Point::new(0, 1)),
        ("D", Point::new(0, -1)),
        ("L", Point::new(-1, 0)),
        ("R", Point::new(1, 0)),
    ]);
    let mut set = HashSet::new();
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);
    set.insert(tail);
    for line in input.lines() {
        let (direction, steps_count) = line.split_once(' ').unwrap();
        let movement = map[direction];
        let steps_count = steps_count.parse::<u32>().unwrap();
        for _ in 0..steps_count {
            let last_head = head;
            head += movement;
            if head.x.abs_diff(tail.x) > 1 || head.y.abs_diff(tail.y) > 1 {
                tail = last_head;
            }
            set.insert(tail);
        }
    }
    set.len().to_string()
}

#[allow(dead_code)]
fn print_rope(rope: &[Point]) {
    let max = rope
        .iter()
        .map(|point| point.x.abs().max(point.y.abs()) as usize)
        .max()
        .unwrap();
    let mut lines = vec![vec!['.'; max * 2 + 1]; max * 2 + 1];
    for (i, part) in rope.iter().enumerate() {
        if lines[(part.y + max as i32) as usize][(part.x + max as i32) as usize] == '.' {
            lines[(part.y + max as i32) as usize][(part.x + max as i32) as usize] =
                i.to_string().chars().next().unwrap();
        }
    }
    for line in lines.iter().rev() {
        println!("{}", line.iter().collect::<String>());
    }
    println!();
}

pub fn part_2(input: &str) -> String {
    let map = HashMap::from([
        ("U", Point::new(0, 1)),
        ("D", Point::new(0, -1)),
        ("L", Point::new(-1, 0)),
        ("R", Point::new(1, 0)),
    ]);
    let mut set = HashSet::new();
    let mut rope = vec![Point::new(0, 0); 10];
    set.insert(rope[rope.len() - 1]);
    for line in input.lines() {
        let (direction, steps_count) = line.split_once(' ').unwrap();
        let movement = map[direction];
        let steps_count = steps_count.parse::<u32>().unwrap();
        for _ in 0..steps_count {
            rope[0] += movement;
            for i in 1..rope.len() {
                let (ahead, behind) = (rope[i - 1], rope.get_mut(i).unwrap());
                if ahead.x.abs_diff(behind.x) > 1 {
                    behind.x += (ahead.x - behind.x).signum();
                    if ahead.y.abs_diff(behind.y) > 0 {
                        behind.y += (ahead.y - behind.y).signum();
                    }
                } else if ahead.y.abs_diff(behind.y) > 1 {
                    behind.y += (ahead.y - behind.y).signum();
                    if ahead.x.abs_diff(behind.x) > 0 {
                        behind.x += (ahead.x - behind.x).signum();
                    }
                }
            }
            set.insert(rope[rope.len() - 1]);
        }
    }
    set.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "13");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "1");
        assert_eq!(part_2(INPUT2), "36");
    }
}
