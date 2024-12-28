use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut start_points = Vec::new();
    let mut matrix: Vec<Vec<u64>> = Vec::new();

    for (r, line) in INPUT.lines().enumerate() {
        let mut buffer = vec![];
        for (c, ch) in line.bytes().enumerate() {
            if ch == b'0' {
                start_points.push((r, c));
            }
            buffer.push((ch - b'0') as u64);
        }
        matrix.push(buffer);
    }

    let total_score: usize = start_points
        .into_iter()
        .map(|start| {
            let mut stack = vec![start];
            let mut score = 0;

            while let Some(current_node) = stack.pop() {
                let next_pos = valid_positions(&matrix, current_node);
                if next_pos.is_empty() {
                    if matrix[current_node.0][current_node.1] == 9 {
                        score += 1;
                    }
                } else {
                    stack.extend_from_slice(&next_pos);
                }
            }

            score
        })
        .sum();

    println!("{:?}", total_score);
}

fn part1() {
    let mut start_points = Vec::new();
    let mut matrix: Vec<Vec<u64>> = Vec::new();

    for (r, line) in INPUT.lines().enumerate() {
        let mut buffer = vec![];
        for (c, ch) in line.bytes().enumerate() {
            if ch == b'0' {
                start_points.push((r, c));
            }
            buffer.push((ch - b'0') as u64);
        }
        matrix.push(buffer);
    }

    let total_score: usize = start_points
        .into_iter()
        .map(|start| {
            let mut stack = vec![start];
            let mut end_pos = HashSet::new();

            while let Some(current_node) = stack.pop() {
                let next_pos = valid_positions(&matrix, current_node);
                if next_pos.is_empty() {
                    if matrix[current_node.0][current_node.1] == 9 {
                        end_pos.insert(current_node);
                    }
                } else {
                    stack.extend_from_slice(&next_pos);
                }
            }

            end_pos.len()
        })
        .sum();

    println!("{:?}", total_score);
}

fn valid_positions(matrix: &[Vec<u64>], pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = vec![];

    if pos.0 > 0 && matrix[pos.0][pos.1] + 1 == matrix[pos.0 - 1][pos.1] {
        result.push((pos.0 - 1, pos.1));
    }

    if pos.1 > 0 && matrix[pos.0][pos.1] + 1 == matrix[pos.0][pos.1 - 1] {
        result.push((pos.0, pos.1 - 1));
    }

    if pos.0 < matrix.len() - 1 && matrix[pos.0][pos.1] + 1 == matrix[pos.0 + 1][pos.1] {
        result.push((pos.0 + 1, pos.1));
    }

    if pos.1 < matrix[0].len() - 1 && matrix[pos.0][pos.1] + 1 == matrix[pos.0][pos.1 + 1] {
        result.push((pos.0, pos.1 + 1));
    }

    result
}
