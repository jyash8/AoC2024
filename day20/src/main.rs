use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut start = None;
    let mut end = None;

    let mut matrix = vec![];

    for (r, line) in INPUT.lines().enumerate() {
        let mut buffer = vec![];
        for (c, ch) in line.chars().enumerate() {
            if ch == 'S' {
                buffer.push('.');
                start = Some((r, c));
            } else if ch == 'E' {
                buffer.push('.');
                end = Some((r, c));
            } else {
                buffer.push(ch);
            }
        }
        matrix.push(buffer);
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let start = start.expect("Start not found.");
    let end = end.expect("Start not found.");

    let mut dists = vec![vec![-1i64; cols]; rows];
    dists[start.0][start.1] = 0;

    let mut cur_pos = start;
    while cur_pos != end {
        for (nr, nc) in adjacent_coords(cur_pos, rows, cols) {
            if matrix[nr][nc] == '#' {
                continue;
            }
            if dists[nr][nc] != -1 {
                continue;
            }
            dists[nr][nc] = dists[cur_pos.0][cur_pos.1] + 1;
            cur_pos = (nr, nc);
        }
    }

    let mut count = 0usize;

    for r in 0..rows {
        for c in 0..cols {
            if matrix[r][c] == '#' {
                continue;
            }
            for radius in 2..21 {
                for dr in 0..=radius {
                    let dc = radius - dr;
                    let r = r as isize;
                    let c = c as isize;

                    for (nr, nc) in HashSet::from([
                        (r + dr, c + dc),
                        (r + dr, c - dc),
                        (r - dr, c + dc),
                        (r - dr, c - dc),
                    ]) {
                        if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize {
                            continue;
                        }
                        if matrix[nr as usize][nc as usize] == '#' {
                            continue;
                        }
                        if dists[nr as usize][nc as usize] - dists[r as usize][c as usize]
                            >= (100 + radius) as i64
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}

fn part1() {
    let mut start = None;
    let mut end = None;

    let mut matrix = vec![];

    for (r, line) in INPUT.lines().enumerate() {
        let mut buffer = vec![];
        for (c, ch) in line.chars().enumerate() {
            if ch == 'S' {
                buffer.push('.');
                start = Some((r, c));
            } else if ch == 'E' {
                buffer.push('.');
                end = Some((r, c));
            } else {
                buffer.push(ch);
            }
        }
        matrix.push(buffer);
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let start = start.expect("Start not found.");
    let end = end.expect("Start not found.");

    let mut dists = vec![vec![-1i64; cols]; rows];
    dists[start.0][start.1] = 0;

    let mut cur_pos = start;
    while cur_pos != end {
        for (nr, nc) in adjacent_coords(cur_pos, rows, cols) {
            if matrix[nr][nc] == '#' {
                continue;
            }
            if dists[nr][nc] != -1 {
                continue;
            }
            dists[nr][nc] = dists[cur_pos.0][cur_pos.1] + 1;
            cur_pos = (nr, nc);
        }
    }

    let mut count = 0usize;

    for r in 0..rows {
        for c in 0..cols {
            if matrix[r][c] == '#' {
                continue;
            }

            for (nr, nc) in cheat_positions((r, c), rows, cols) {
                if matrix[nr][nc] == '#' {
                    continue;
                }

                if dists[r][c].abs_diff(dists[nr][nc]) >= 102 {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

fn cheat_positions(coord: (usize, usize), rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let coord = (coord.0 as isize, coord.1 as isize);
    [
        (coord.0 + 2, coord.1),
        (coord.0 + 1, coord.1 + 1),
        (coord.0, coord.1 + 2),
        (coord.0 - 1, coord.1 + 1),
    ]
    .into_iter()
    .filter(|&(r, c)| !(r < 0 || r >= rows as isize || c < 0 || c >= cols as isize))
    .map(|num| (num.0 as usize, num.1 as usize))
    .collect()
}

fn adjacent_coords(coord: (usize, usize), rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];

    if coord.0 > 0 {
        result.push((coord.0 - 1, coord.1));
    }
    if coord.0 < rows - 1 {
        result.push((coord.0 + 1, coord.1));
    }
    if coord.1 > 0 {
        result.push((coord.0, coord.1 - 1));
    }
    if coord.1 < cols - 1 {
        result.push((coord.0, coord.1 + 1));
    }

    result
}

fn print_matrix(matrix: &[Vec<char>]) {
    for line in matrix {
        for ch in line {
            print!("{}", ch)
        }
        println!()
    }
}
