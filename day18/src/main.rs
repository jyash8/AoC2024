use std::collections::{HashSet, VecDeque};

// const INPUT: &str = include_str!("../test.txt");
// const SIZE: usize = 6;
// const FIRST: usize = 12;

const INPUT: &str = include_str!("../input.txt");
const SIZE: usize = 70;
const FIRST: usize = 1024;

fn main() {
    part1();
    part2();
}

fn part2() {
    let bytes: Vec<(usize, usize)> = INPUT
        .lines()
        .map(|line| {
            let mut comma_split = line.split(",");
            (
                comma_split.next().unwrap().parse().unwrap(),
                comma_split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    // println!("{:?}", bytes);

    let mut grid = [['.'; SIZE + 1]; SIZE + 1];

    for &(x, y) in bytes.iter() {
        grid[y][x] = '#';
        if !is_path(&grid) {
            println!("{},{}", x, y);
            break;
        }
    }
}

fn is_path(grid: &[[char; SIZE + 1]; SIZE + 1]) -> bool {
    let mut stack = Vec::new();
    stack.push((0, 0));

    let mut set = HashSet::new();

    while let Some(pos) = stack.pop() {
        if pos == (SIZE, SIZE) {
            return true;
        }

        if !set.insert(pos) {
            continue;
        }

        for next_pos in next_cells(grid, pos) {
            stack.push(next_pos);
        }
    }
    false
}

fn part1() {
    let bytes: Vec<(usize, usize)> = INPUT
        .lines()
        .map(|line| {
            let mut comma_split = line.split(",");
            (
                comma_split.next().unwrap().parse().unwrap(),
                comma_split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    // println!("{:?}", bytes);

    let mut grid = [['.'; SIZE + 1]; SIZE + 1];

    for &(x, y) in bytes[..FIRST].iter() {
        grid[y][x] = '#';
    }

    // for line in grid {
    //     for ch in line {
    //         print!("{ch}");
    //     }
    //     println!()
    // }

    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), 0));

    let mut set = HashSet::new();

    while let Some((pos, step)) = queue.pop_front() {
        if pos == (SIZE, SIZE) {
            println!("{}", step);
            break;
        }

        if !set.insert(pos) {
            continue;
        }

        for (next_pos, next_step) in next_cells_with_step(&grid, pos, step) {
            queue.push_back((next_pos, next_step));
        }
    }
}

fn next_cells_with_step(
    grid: &[[char; SIZE + 1]; SIZE + 1],
    (x, y): (usize, usize),
    step: usize,
) -> Vec<((usize, usize), usize)> {
    let mut result = vec![];

    if x > 0 && grid[y][x - 1] == '.' {
        result.push(((x - 1, y), step + 1));
    }
    if y > 0 && grid[y - 1][x] == '.' {
        result.push(((x, y - 1), step + 1));
    }

    if x < grid[0].len() - 1 && grid[y][x + 1] == '.' {
        result.push(((x + 1, y), step + 1));
    }
    if y < grid.len() - 1 && grid[y + 1][x] == '.' {
        result.push(((x, y + 1), step + 1));
    }

    result
}

fn next_cells(grid: &[[char; SIZE + 1]; SIZE + 1], (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = vec![];

    if x > 0 && grid[y][x - 1] == '.' {
        result.push((x - 1, y));
    }
    if y > 0 && grid[y - 1][x] == '.' {
        result.push((x, y - 1));
    }

    if x < grid[0].len() - 1 && grid[y][x + 1] == '.' {
        result.push((x + 1, y));
    }
    if y < grid.len() - 1 && grid[y + 1][x] == '.' {
        result.push((x, y + 1));
    }

    result
}
