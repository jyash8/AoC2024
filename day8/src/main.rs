use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut positions: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    let mut matrix = vec![];
    for (r, line) in INPUT.lines().enumerate() {
        let mut buffer = vec![];
        for (c, ch) in line.chars().enumerate() {
            if ch.is_ascii_alphabetic() || ch.is_ascii_digit() {
                positions
                    .entry(ch)
                    .or_default()
                    .push((r as isize, c as isize));
            }
            buffer.push(ch);
        }
        matrix.push(buffer);
    }

    let rows = matrix.len() as isize;
    let cols = matrix[0].len() as isize;

    let v: Vec<(isize, isize)> = positions
        .into_iter()
        .flat_map(|(_k, v)| {
            v.into_iter().combinations(2).flat_map(move |comb| {
                let mut diff = (comb[0].0 - comb[1].0, comb[0].1 - comb[1].1);
                let gcd = gcd(diff.0, diff.1);
                diff.0 /= gcd;
                diff.1 /= gcd;
                let mut buffer = vec![comb[0], comb[1]];

                let mut current_pos = comb[0];

                loop {
                    let new_pos = (current_pos.0 + diff.0, current_pos.1 + diff.1);
                    if new_pos.0 < 0
                        || new_pos.0 > rows - 1
                        || new_pos.1 < 0
                        || new_pos.1 > cols - 1
                    {
                        break;
                    } else {
                        buffer.push(new_pos);
                    }

                    current_pos = new_pos;
                }

                current_pos = comb[1];

                loop {
                    let new_pos = (current_pos.0 - diff.0, current_pos.1 - diff.1);
                    if new_pos.0 < 0
                        || new_pos.0 > rows - 1
                        || new_pos.1 < 0
                        || new_pos.1 > cols - 1
                    {
                        break;
                    } else {
                        buffer.push(new_pos);
                    }

                    current_pos = new_pos;
                }

                buffer
            })
        })
        .filter(|pos| !(pos.0 < 0 || pos.0 > rows - 1 || pos.1 < 0 || pos.1 > cols - 1))
        .unique()
        .collect();

    println!("{}", v.len());
}

fn gcd(mut a: isize, mut b: isize) -> isize {
    a = a.abs();
    b = b.abs();

    // If equal, return any of them
    if a == b {
        return a;
    }

    // Swap a with b, if b is greater than a
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }

    while b > 0 {
        // This is the trickiest part
        // We swap a with b, and b with a%b, till b becomes 0
        let temp = a;
        a = b;
        b = temp % b;
    }

    // Now, a%b = 0, hence return it
    a
}

fn part1() {
    let mut positions: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    let mut matrix = vec![];
    for (r, line) in INPUT.lines().enumerate() {
        let mut buffer = vec![];
        for (c, ch) in line.chars().enumerate() {
            if ch.is_ascii_alphabetic() || ch.is_ascii_digit() {
                positions
                    .entry(ch)
                    .or_default()
                    .push((r as isize, c as isize));
            }
            buffer.push(ch);
        }
        matrix.push(buffer);
    }

    let rows = matrix.len() as isize;
    let cols = matrix[0].len() as isize;

    let v: Vec<(isize, isize)> = positions
        .into_iter()
        .flat_map(|(k, v)| {
            v.into_iter().combinations(2).flat_map(move |comb| {
                let diff = (comb[0].0 - comb[1].0, comb[0].1 - comb[1].1);
                [
                    (k, (comb[0].0 + diff.0, comb[0].1 + diff.1)),
                    (k, (comb[1].0 - diff.0, comb[1].1 - diff.1)),
                ]
            })
        })
        .filter_map(|(_, pos)| {
            if pos.0 < 0 || pos.0 > rows - 1 || pos.1 < 0 || pos.1 > cols - 1 {
                None
            } else {
                Some(pos)
            }
        })
        .unique()
        .collect();

    println!("{}", v.len());
}
