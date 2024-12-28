const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut line_split = INPUT.split("\n\n");
    let mut robot_pos = None;

    let mut matrix: Vec<Vec<char>> = line_split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|ch| match ch {
                    '.' => ['.', '.'],
                    '#' => ['#', '#'],
                    '@' => ['@', '.'],
                    'O' => ['[', ']'],
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    'outer: for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            if matrix[r][c] == '@' {
                robot_pos = Some((r, c));
                break 'outer;
            }
        }
    }

    let directions: Vec<(isize, isize)> = line_split
        .next()
        .unwrap()
        .trim()
        .chars()
        .flat_map(|ch| match ch {
            '<' => Some((0, -1)),
            '>' => Some((0, 1)),
            '^' => Some((-1, 0)),
            'v' => Some((1, 0)),
            _ => None,
        })
        .collect();

    let (mut r, mut c) = robot_pos.unwrap();

    for (dr, dc) in directions {
        let mut targets = vec![(r, c)];
        let mut stack = vec![(r, c)];
        let mut go = true;

        while let Some((cr, cc)) = stack.pop() {
            let nr = (cr as isize + dr) as usize;
            let nc = (cc as isize + dc) as usize;

            if targets.contains(&(nr, nc)) {
                continue;
            }

            let ch = matrix[nr][nc];

            if ch == '#' {
                go = false;
                break;
            }

            if ch == '[' {
                targets.push((nr, nc));
                targets.push((nr, nc + 1));
                stack.push((nr, nc));
                stack.push((nr, nc + 1));
            }

            if ch == ']' {
                targets.push((nr, nc));
                targets.push((nr, nc - 1));
                stack.push((nr, nc));
                stack.push((nr, nc - 1));
            }
        }

        if !go {
            continue;
        }

        let copy_mat = matrix.clone();

        matrix[r][c] = '.';
        matrix[(r as isize + dr) as usize][(c as isize + dc) as usize] = '@';
        for &(br, bc) in targets[1..].iter() {
            matrix[br][bc] = '.'
        }
        for &(br, bc) in targets[1..].iter() {
            matrix[(br as isize + dr) as usize][(bc as isize + dc) as usize] = copy_mat[br][bc];
        }
        r = (r as isize + dr) as usize;
        c = (c as isize + dc) as usize;
    }

    let sum: usize = matrix
        .iter()
        .enumerate()
        .map(|(r, line)| {
            line.iter()
                .enumerate()
                .map(|(c, ch)| if *ch == '[' { r * 100 + c } else { 0 })
                .sum::<usize>()
        })
        .sum();

    println!("{}", sum);
}

fn part1() {
    let mut line_split = INPUT.split("\n\n");
    let mut robot_pos = None;

    let mut matrix: Vec<Vec<char>> = line_split
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, ch)| {
                    if ch == '@' {
                        robot_pos = Some((r, c));
                    }
                    ch
                })
                .collect()
        })
        .collect();

    let directions: Vec<Direction> = line_split
        .next()
        .unwrap()
        .trim()
        .chars()
        .flat_map(|ch| match ch {
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            _ => None,
        })
        .collect();

    // for line in matrix.iter() {
    //     for ch in line {
    //         print!("{ch}")
    //     }
    //     println!()
    // }

    let mut robot_pos = robot_pos.unwrap();

    // println!("Current pos: {:?}", robot_pos);
    for dir in directions {
        let possible = is_possible(&matrix, robot_pos, dir);

        if let Some(((y, x), is_shift)) = possible {
            if is_shift {
                matrix[y][x] = 'O';
                matrix[robot_pos.0][robot_pos.1] = '.';
                robot_pos = next_pos(robot_pos, dir);
                matrix[robot_pos.0][robot_pos.1] = '@';
            } else {
                matrix[robot_pos.0][robot_pos.1] = '.';
                robot_pos = next_pos(robot_pos, dir);
                // assert_eq!(robot_pos, (y, x));
                matrix[robot_pos.0][robot_pos.1] = '@';
            }
            // println!("{:?}", dir);
        } else {
            // println!("{:?} no shift", dir);
        }

        // println!("Current pos: {:?}", robot_pos);
        // for line in matrix.iter() {
        //     for ch in line {
        //         print!("{ch}")
        //     }
        //     println!()
        // }
    }

    let sum: usize = matrix
        .iter()
        .enumerate()
        .map(|(r, line)| {
            line.iter()
                .enumerate()
                .map(|(c, ch)| if *ch == 'O' { r * 100 + c } else { 0 })
                .sum::<usize>()
        })
        .sum();

    println!("{}", sum);
}

#[inline]
fn next_pos((y, x): (usize, usize), dir: Direction) -> (usize, usize) {
    match dir {
        Direction::Up => (y - 1, x),
        Direction::Down => (y + 1, x),
        Direction::Left => (y, x - 1),
        Direction::Right => (y, x + 1),
    }
}

fn is_possible(
    matrix: &[Vec<char>],
    robot_pos: (usize, usize),
    dir: Direction,
) -> Option<((usize, usize), bool)> {
    let mut is_shift = false;

    match dir {
        Direction::Up => {
            for r in (0..=robot_pos.0).rev() {
                if matrix[r][robot_pos.1] == '#' {
                    return None;
                } else if matrix[r][robot_pos.1] == '.' {
                    return Some(((r, robot_pos.1), is_shift));
                } else if matrix[r][robot_pos.1] == 'O' {
                    is_shift = true;
                }
            }
            unreachable!()
        }
        Direction::Down => {
            #[allow(clippy::needless_range_loop)]
            for r in robot_pos.0..=matrix.len() - 1 {
                if matrix[r][robot_pos.1] == '#' {
                    return None;
                } else if matrix[r][robot_pos.1] == '.' {
                    return Some(((r, robot_pos.1), is_shift));
                } else if matrix[r][robot_pos.1] == 'O' {
                    is_shift = true;
                }
            }
            unreachable!()
        }
        Direction::Left => {
            for c in (0..=robot_pos.1).rev() {
                if matrix[robot_pos.0][c] == '#' {
                    return None;
                } else if matrix[robot_pos.0][c] == '.' {
                    return Some(((robot_pos.0, c), is_shift));
                } else if matrix[robot_pos.0][c] == 'O' {
                    is_shift = true;
                }
            }
            unreachable!()
        }
        Direction::Right => {
            for c in robot_pos.1..=matrix[0].len() - 1 {
                if matrix[robot_pos.0][c] == '#' {
                    return None;
                } else if matrix[robot_pos.0][c] == '.' {
                    return Some(((robot_pos.0, c), is_shift));
                } else if matrix[robot_pos.0][c] == 'O' {
                    is_shift = true;
                }
            }
            unreachable!()
        }
    }
}
