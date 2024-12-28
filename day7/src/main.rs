use rayon::iter::ParallelIterator;
use rayon::str::ParallelString;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

fn part2() {
    let result: u64 = INPUT
        .par_lines()
        .map(|line| {
            let mut colon_split = line.split(':');

            let key: u64 = colon_split.next().unwrap().parse::<u64>().unwrap();

            let val: Vec<u64> = colon_split
                .next()
                .unwrap()
                .par_split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            (key, val)
        })
        .filter_map(|(k, v)| {
            for mut i in 0..(3_u64.pow((v.len() - 1) as u32)) {
                let mut acc = v[0];
                for j in 0..v.len() - 1 {
                    // Check if the j-th bit is set in i
                    match i % 3 {
                        0 => acc += v[j + 1],
                        1 => acc *= v[j + 1],
                        2 => {
                            acc = (format!("{}", acc) + &format!("{}", v[j + 1]))
                                .parse()
                                .unwrap()
                        }
                        _ => unreachable!(),
                    }
                    i /= 3;
                }
                if acc == k {
                    return Some(acc);
                }
            }
            None
        })
        .sum();

    println!("{}", result);
}

#[allow(dead_code)]
fn part1() {
    let result: u64 = INPUT
        .par_lines()
        .map(|line| {
            let mut colon_split = line.split(':');

            let key: u64 = colon_split.next().unwrap().parse::<u64>().unwrap();

            let val: Vec<u64> = colon_split
                .next()
                .unwrap()
                .par_split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect();

            (key, val)
        })
        .filter_map(|(k, v)| {
            for i in 0..(1 << (v.len() - 1)) {
                let mut acc = v[0];
                for j in 0..v.len() - 1 {
                    // Check if the j-th bit is set in i
                    if (i & (1 << j)) != 0 {
                        acc += v[j + 1];
                    } else {
                        acc *= v[j + 1];
                    }
                }
                if acc == k {
                    return Some(acc);
                }
            }
            None
        })
        .sum();

    println!("{}", result);
}
