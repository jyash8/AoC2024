use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

fn part2() {
    let num_keypad = vec![
        vec![Some('7'), Some('8'), Some('9')],
        vec![Some('4'), Some('5'), Some('6')],
        vec![Some('1'), Some('2'), Some('3')],
        vec![None, Some('0'), Some('A')],
    ];
    let num_seqs = compute_seqs(&num_keypad);

    let dir_keypad = vec![
        vec![None, Some('^'), Some('A')],
        vec![Some('<'), Some('v'), Some('>')],
    ];
    let dir_seqs = compute_seqs(&dir_keypad);
    let dir_lengths: HashMap<(char, char), usize> =
        dir_seqs.iter().map(|(k, v)| (*k, v[0].len())).collect();

    let mut total = 0;

    let mut memo = HashMap::new();

    for line in INPUT.lines() {
        // println!("{}", line);
        let inputs = solve(line.to_string(), &num_seqs);
        let length = inputs
            .into_iter()
            .map(|seq| compute_length(seq, 25, &dir_seqs, &dir_lengths, &mut memo))
            .min()
            .unwrap();
        total += length * line[..line.len() - 1].parse::<usize>().unwrap();
    }

    println!("{:?}", total);
}

fn compute_length(
    seq: String,
    depth: usize,
    dir_seqs: &HashMap<(char, char), Vec<String>>,
    dir_lengths: &HashMap<(char, char), usize>,
    memo: &mut HashMap<(String, usize), usize>,
) -> usize {
    // Checking if value exists in cache and
    // returning it if it does
    if memo.contains_key(&(seq.clone(), depth)) {
        return memo[&(seq.clone(), depth)];
    }

    if depth == 1 {
        let ret_val = (String::from('A') + &seq)
            .chars()
            .zip(seq.chars())
            .map(|(x, y)| dir_lengths[&(x, y)])
            .sum();
        memo.insert((seq, depth), ret_val);
        return ret_val;
    }

    let mut length = 0;
    for (x, y) in (String::from("A") + &seq).chars().zip(seq.chars()) {
        length += dir_seqs[&(x, y)]
            .iter()
            .map(|subseq| compute_length(subseq.clone(), depth - 1, dir_seqs, dir_lengths, memo))
            .min()
            .unwrap();
    }

    // Storing the computation
    memo.insert((seq, depth), length);

    length
}

fn part1() {
    let num_keypad = vec![
        vec![Some('7'), Some('8'), Some('9')],
        vec![Some('4'), Some('5'), Some('6')],
        vec![Some('1'), Some('2'), Some('3')],
        vec![None, Some('0'), Some('A')],
    ];

    let num_seqs = compute_seqs(&num_keypad);

    let dir_keypad = vec![
        vec![None, Some('^'), Some('A')],
        vec![Some('<'), Some('v'), Some('>')],
    ];
    let dir_seqs = compute_seqs(&dir_keypad);

    let mut total = 0;

    for line in INPUT.lines() {
        // println!("{}", line);
        let mut cur_robot = solve(line.to_string(), &num_seqs);
        for _ in 0..2 {
            let mut possible_next = vec![];
            for seq in cur_robot {
                possible_next.extend_from_slice(&solve(seq, &dir_seqs));
            }
            let mut min_len = usize::MAX;
            for pos in possible_next.iter() {
                min_len = pos.len().min(min_len);
            }
            possible_next.retain(|string| string.len() == min_len);
            cur_robot = possible_next;
        }

        let length = cur_robot[0].len();
        let complexity = length * line[..line.len() - 1].parse::<usize>().unwrap();

        total += complexity;
    }

    println!("{:?}", total);
}

fn compute_seqs(keypad: &[Vec<Option<char>>]) -> HashMap<(char, char), Vec<String>> {
    let mut pos = HashMap::new();

    let rows = keypad.len() as isize;
    let cols = keypad[0].len() as isize;

    for (r, line) in keypad.iter().enumerate() {
        for c in 0..line.len() {
            if let Some(ch) = keypad[r][c] {
                pos.insert(ch, (r as isize, c as isize));
            }
        }
    }

    let mut seqs = HashMap::new();
    for x in pos.keys() {
        for y in pos.keys() {
            if x == y {
                seqs.insert((*x, *y), vec![String::from("A")]);
                continue;
            }

            let mut possibilities = vec![];
            let mut optimal = usize::MAX;
            let mut q: VecDeque<((isize, isize), String)> =
                VecDeque::from([(pos[x], String::new())]);

            'd: while let Some(((r, c), moves)) = q.pop_front() {
                for (nr, nc, nm) in [
                    (r - 1, c, "^"),
                    (r + 1, c, "v"),
                    (r, c - 1, "<"),
                    (r, c + 1, ">"),
                ] {
                    if nr < 0 || nc < 0 || nr >= rows || nc >= cols {
                        continue;
                    }
                    if let Some(ch) = keypad.index_i(nr).index_i(nc) {
                        if ch == y {
                            if optimal < moves.len() + 1 {
                                break 'd;
                            }

                            optimal = moves.len() + 1;
                            possibilities.push(moves.clone() + nm + "A");
                        } else {
                            let mut new_moves = moves.clone();
                            new_moves += nm;
                            q.push_back(((nr, nc), new_moves));
                        }
                    } else {
                        continue;
                    }
                }
            }

            seqs.insert((*x, *y), possibilities);
        }
    }

    seqs
}

fn solve(string: String, seqs: &HashMap<(char, char), Vec<String>>) -> Vec<String> {
    (String::from("A") + &string)
        .chars()
        .zip(string.chars())
        .map(|(x, y)| seqs[&(x, y)].clone())
        .multi_cartesian_product()
        .map(|v| v.join(""))
        .collect()
}

trait GetByIsize<T> {
    fn get_i(&self, index: isize) -> Option<&T>;
    fn index_i(&self, idx: isize) -> &T;
}

impl<T> GetByIsize<T> for [T] {
    fn get_i(&self, index: isize) -> Option<&T> {
        if index < 0 {
            return None;
        }

        self.get(index as usize)
    }

    fn index_i(&self, idx: isize) -> &T {
        self.get_i(idx).unwrap()
    }
}
