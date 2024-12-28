use std::collections::{HashMap, HashSet};

use rayon::{iter::ParallelIterator, str::ParallelString};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut seq_to_total = HashMap::new();

    INPUT
        .lines()
        .map(|raw_num| raw_num.parse::<i64>().unwrap())
        .for_each(|mut num| {
            let mut buyer = vec![num % 10];
            for _ in 0..2000 {
                num = ((num * 64) ^ num) % 16777216;
                num = ((num / 32) ^ num) % 16777216;
                num = ((num * 2048) ^ num) % 16777216;
                buyer.push(num % 10);
            }

            let mut seen = HashSet::new();
            for i in 0..(buyer.len() - 4) {
                let diffs: Vec<i64> = buyer[i..i + 5]
                    .windows(2)
                    .map(|slice| slice[1] - slice[0])
                    .collect();
                if seen.contains(&diffs) {
                    continue;
                }
                seen.insert(diffs.clone());
                let end = buyer[i + 4];
                *seq_to_total.entry(diffs).or_insert(0) += end;
            }
        });

    println!("{:?}", seq_to_total.values().max());
}

fn part1() {
    let sum = INPUT
        .par_lines()
        .map(|raw_num| raw_num.parse::<u64>().unwrap())
        .map(|mut num| {
            for _ in 0..2000 {
                num = ((num * 64) ^ num) % 16777216;
                num = ((num / 32) ^ num) % 16777216;
                num = ((num * 2048) ^ num) % 16777216;
            }
            num
        })
        .sum::<u64>();

    println!("{sum}");
}
