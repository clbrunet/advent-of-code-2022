pub fn part_1(input: &str) {
    let mut acc = 0;
    let mut max = 0;
    for line in input.lines() {
        match line.parse::<i32>() {
            Ok(value) => acc += value,
            Err(_) => {
                if max < acc {
                    max = acc;
                }
                acc = 0;
            }
        }
    }
    if max < acc {
        max = acc;
    }
    println!("Part 1 : {}", max);
}

fn part_2_updates_maximums(max1: &mut i32, max2: &mut i32, max3: &mut i32, acc: i32) {
    if *max1 < acc {
        *max3 = *max2;
        *max2 = *max1;
        *max1 = acc;
    } else if *max2 < acc {
        *max3 = *max2;
        *max2 = acc;
    } else if *max3 < acc {
        *max3 = acc;
    }
}

pub fn part_2(input: &str) {
    let mut acc = 0;
    let mut max1 = 0;
    let mut max2 = 0;
    let mut max3 = 0;
    for line in input.lines() {
        match line.parse::<i32>() {
            Ok(value) => acc += value,
            Err(_) => {
                part_2_updates_maximums(&mut max1, &mut max2, &mut max3, acc);
                acc = 0;
            }
        }
    }
    part_2_updates_maximums(&mut max1, &mut max2, &mut max3, acc);
    println!("Part 2 : {}", max1 + max2 + max3);
}
