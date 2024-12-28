use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut line_split = INPUT.split("\n\n");

    let available: HashSet<&[u8]> = line_split
        .next()
        .unwrap()
        .split(", ")
        .map(|string| string.as_bytes())
        .collect();

    let mut memo = HashMap::with_capacity(1000);

    let needs_count: usize = line_split
        .next()
        .unwrap()
        .trim()
        .split('\n')
        // .map(|need| get_comb_num(need.as_bytes(), &available))
        .map(|need| get_comb_num_memo(need.as_bytes(), &available, &mut memo))
        .sum();

    println!("{}", needs_count);
}

fn get_comb_num_memo<'a>(
    cur_stripes: &'a [u8],
    available: &HashSet<&'a [u8]>,
    memo: &mut HashMap<&'a [u8], usize>,
) -> usize {
    if memo.contains_key(cur_stripes) {
        return memo[cur_stripes];
    }

    let mut ways = 0;
    if cur_stripes.is_empty() {
        ways = 1;
    }
    for &stripe in available {
        if cur_stripes.len() >= stripe.len() && &cur_stripes[0..stripe.len()] == stripe {
            let comb = get_comb_num_memo(&cur_stripes[stripe.len()..], available, memo);
            ways += comb;
        }
    }

    memo.insert(cur_stripes, ways);
    ways
}

// Useful for debugging not for actual code
#[allow(dead_code)]
fn get_combinations<'a>(
    cur_stripes: &'a [u8],
    available: &HashSet<&'a [u8]>,
) -> Vec<Vec<&'a [u8]>> {
    if cur_stripes.is_empty() {
        return vec![vec![]];
    }

    let mut ways = vec![];

    for &stripe in available {
        if cur_stripes.len() >= stripe.len() && &cur_stripes[0..stripe.len()] == stripe {
            let comb = get_combinations(&cur_stripes[stripe.len()..], available);
            for mut way in comb {
                way.push(stripe);
                ways.push(way);
            }
        }
    }
    ways
}

fn part1() {
    let mut line_split = INPUT.split("\n\n");

    let available: HashSet<&[u8]> = line_split
        .next()
        .unwrap()
        .split(", ")
        .map(|string| string.as_bytes())
        .collect();

    let needs = line_split
        .next()
        .unwrap()
        .trim()
        .split('\n')
        .filter(|current| can_make(current.as_bytes(), &available))
        .count();

    println!("{}", needs);
}

fn can_make(current: &[u8], available: &HashSet<&[u8]>) -> bool {
    if current.is_empty() {
        return true;
    }

    if available.contains(current) {
        return true;
    }

    for &stripes in available {
        if current.len() >= stripes.len() && &current[0..stripes.len()] == stripes {
            let sub = can_make(&current[stripes.len()..], available);
            if sub {
                return true;
            }
        }
    }

    false
}
