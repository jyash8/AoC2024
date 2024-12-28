use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const INPUT: &str = include_str!("../input.txt");

fn stone_count(stone: u64, steps: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if let Some(val) = memo.get(&(stone, steps)) {
        return *val;
    }

    let return_val;

    if steps == 0 {
        return_val = 1;
    } else if stone == 0 {
        return_val = stone_count(1, steps - 1, memo);
    } else {
        let dig_count = digit_count(stone);
        if dig_count % 2 == 0 {
            let power_10 = 10u64.pow(dig_count / 2);
            return_val = stone_count(stone / power_10, steps - 1, memo)
                + stone_count(stone % power_10, steps - 1, memo);
        } else {
            return_val = stone_count(stone * 2024, steps - 1, memo)
        }
    }

    memo.insert((stone, steps), return_val);

    return_val
}

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut memo = HashMap::new();

    let stones: u64 = INPUT
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .map(|stone| stone_count(stone, 75, &mut memo))
        .sum();

    println!("{}", stones);
}

// Part 2 requires more clever solution
fn part1() {
    let mut stones: Vec<u64> = INPUT
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();

    for _ in 0..25 {
        stones = stones
            .par_iter()
            .flat_map(|num| {
                if num == &0 {
                    vec![1]
                } else {
                    let d_count = digit_count(*num);
                    if d_count % 2 == 0 {
                        let new_digit_count = d_count / 2;
                        let power_10 = 10u64.pow(new_digit_count);
                        vec![num / power_10, num % power_10]
                    } else {
                        vec![num * 2024]
                    }
                }
            })
            .collect();
    }

    println!("{:?}", stones.len());
}

fn digit_count(mut num: u64) -> u32 {
    let mut digit = 0;
    while num > 0 {
        digit += 1;
        num /= 10;
    }

    digit
}
