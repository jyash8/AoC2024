#![allow(clippy::type_complexity)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn turn_left(&self) -> Dir {
        match self {
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
        }
    }

    fn turn_right(&self) -> Dir {
        match self {
            Dir::Right => Dir::Down,
            Dir::Left => Dir::Up,
            Dir::Down => Dir::Left,
            Dir::Up => Dir::Right,
        }
    }
}

fn new_pos(pos: (usize, usize), dir: Dir, (rows, cols): (usize, usize)) -> Option<(usize, usize)> {
    match dir {
        Dir::Up if pos.0 > 0 => Some((pos.0 - 1, pos.1)),
        Dir::Up => None,
        Dir::Down if pos.0 < rows - 1 => Some((pos.0 + 1, pos.1)),
        Dir::Down => None,
        Dir::Left if pos.1 > 0 => Some((pos.0, pos.1 - 1)),
        Dir::Left => None,
        Dir::Right if pos.1 < cols - 1 => Some((pos.0, pos.1 + 1)),
        Dir::Right => None,
    }
}

fn main() {
    part1();
    part2();
}

fn part2() {
    let matrix: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut start = None;

    #[allow(clippy::needless_range_loop)]
    for r in 0..rows {
        for c in 0..cols {
            if matrix[r][c] == 'S' {
                start = Some((r, c));
            }
        }
    }

    let start = start.unwrap();
    let mut pq = BinaryHeap::from([Reverse((0, start, Dir::Right, None))]);
    let mut lowest_cost = HashMap::from([((start, Dir::Right), 0)]);
    let mut backtrack: HashMap<_, HashSet<Option<((usize, usize), Dir)>>> = HashMap::new();
    let mut end_states: HashSet<_> = HashSet::new();
    let mut best_cost = u64::MAX;

    while let Some(Reverse((cost, pos, dir, prev_state))) = pq.pop() {
        if cost > *lowest_cost.get(&(pos, dir)).unwrap_or(&u64::MAX) {
            continue;
        }

        lowest_cost.insert((pos, dir), cost);

        if matrix[pos.0][pos.1] == 'E' {
            if cost > best_cost {
                break;
            }
            best_cost = cost;
            end_states.insert((pos, dir));
        }

        backtrack.entry((pos, dir)).or_default().insert(prev_state);

        for (new_cost, (nr, nc), new_dir) in [
            new_pos(pos, dir, (rows, cols)).map(|pos| (cost + 1, pos, dir)),
            Some((cost + 1000, pos, dir.turn_left())),
            Some((cost + 1000, pos, dir.turn_right())),
        ]
        .into_iter()
        .flatten()
        {
            if matrix[nr][nc] == '#' {
                continue;
            }

            if cost > *lowest_cost.get(&(pos, dir)).unwrap_or(&u64::MAX) {
                continue;
            }

            pq.push(Reverse((new_cost, (nr, nc), new_dir, Some((pos, dir)))));
        }
    }

    let mut states = VecDeque::from_iter(end_states.iter().map(|state| Some(*state)));
    let mut seen: HashSet<_> = end_states.iter().map(|state| Some(*state)).collect();

    while let Some(key) = states.pop_front() {
        if let Some(iter) = key.map(|key| backtrack[&key].iter()) {
            for last in iter {
                if last.map(|last| seen.contains(&Some(last))).unwrap_or(false) {
                    continue;
                }
                seen.insert(*last);
                states.push_back(*last);
            }
        }
    }

    println!(
        "{:?}",
        seen.iter().flat_map(|a| a.map(|a| a.0)).unique().count()
    );
}

#[allow(dead_code)]
fn part1() {
    let matrix: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut start = None;

    #[allow(clippy::needless_range_loop)]
    for r in 0..rows {
        for c in 0..cols {
            if matrix[r][c] == 'S' {
                start = Some((r, c));
            }
        }
    }

    let start = start.unwrap();

    let mut pq = BinaryHeap::from([Reverse((0, start, Dir::Right))]);

    let mut seen = HashSet::from([(start, Dir::Right)]);

    while let Some(Reverse((cost, pos, dir))) = pq.pop() {
        seen.insert((pos, dir));

        if matrix[pos.0][pos.1] == 'E' {
            println!("{}", cost);
            break;
        }

        for (new_cost, (nr, nc), new_dir) in [
            new_pos(pos, dir, (rows, cols)).map(|pos| (cost + 1, pos, dir)),
            Some((cost + 1000, pos, dir.turn_left())),
            Some((cost + 1000, pos, dir.turn_right())),
        ]
        .into_iter()
        .flatten()
        {
            if matrix[nr][nc] == '#' {
                continue;
            }

            if seen.contains(&((nr, nc), new_dir)) {
                continue;
            }

            pq.push(Reverse((new_cost, (nr, nc), new_dir)));
        }
    }
}
