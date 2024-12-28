use std::collections::HashSet;

const INPUT: &str = include_str!("../test.txt");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Dir {
    Null,
    Up,
    Left,
    Right,
    Down,
}

impl Dir {
    fn turn(&self) -> Dir {
        match self {
            Dir::Null => Dir::Null,
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: (usize, usize),
    dir: Dir,
}

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut matrix = vec![];
    let mut pos = None;
    let mut dir = Dir::Null;

    for (r, line) in INPUT.lines().enumerate() {
        let mut buffer = vec![];
        for (c, ch) in line.chars().enumerate() {
            if ch == '^' {
                pos = Some((r, c));
                dir = Dir::Up;
            }
            buffer.push(ch);
        }
        matrix.push(buffer);
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut pos = pos.expect("Position not found");

    matrix[pos.0][pos.1] = '.';

    let original_guard_position = pos;

    let mut loop_obstacle_set = HashSet::new();

    let mut tested = HashSet::new();
    tested.insert(pos);

    loop {
        match dir {
            Dir::Null => break,
            Dir::Up => {
                if pos.0 > 0 {
                    if matrix[pos.0 - 1][pos.1] == '#' {
                        dir = dir.turn();
                    } else {
                        pos.0 -= 1;
                        if tested.insert(pos) && is_loop(&mut matrix, pos, original_guard_position)
                        {
                            loop_obstacle_set.insert(pos);
                        }
                    }
                } else {
                    break;
                }
            }

            Dir::Down => {
                if pos.0 < rows - 1 {
                    if matrix[pos.0 + 1][pos.1] == '#' {
                        dir = dir.turn();
                    } else {
                        pos.0 += 1;
                        if tested.insert(pos) && is_loop(&mut matrix, pos, original_guard_position)
                        {
                            loop_obstacle_set.insert(pos);
                        }
                    }
                } else {
                    break;
                }
            }

            Dir::Left => {
                if pos.1 > 0 {
                    if matrix[pos.0][pos.1 - 1] == '#' {
                        dir = dir.turn();
                    } else {
                        pos.1 -= 1;
                        if tested.insert(pos) && is_loop(&mut matrix, pos, original_guard_position)
                        {
                            loop_obstacle_set.insert(pos);
                        }
                    }
                } else {
                    break;
                }
            }

            Dir::Right => {
                if pos.1 < cols - 1 {
                    if matrix[pos.0][pos.1 + 1] == '#' {
                        dir = dir.turn();
                    } else {
                        pos.1 += 1;
                        if tested.insert(pos) && is_loop(&mut matrix, pos, original_guard_position)
                        {
                            loop_obstacle_set.insert(pos);
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }

    println!("Loop: {}", loop_obstacle_set.len());
}

fn is_loop(
    matrix: &mut [Vec<char>],
    new_obstacle: (usize, usize),
    original_guard_position: (usize, usize),
) -> bool {
    let mut visited = HashSet::new();
    visited.insert(State {
        pos: original_guard_position,
        dir: Dir::Up,
    });

    let rows = matrix.len();
    let cols = matrix[0].len();

    matrix[new_obstacle.0][new_obstacle.1] = '#';

    let mut state = State {
        pos: original_guard_position,
        dir: Dir::Up,
    };

    let mut is_loop = false;

    loop {
        match state.dir {
            Dir::Null => break,
            Dir::Up => {
                if state.pos.0 > 0 {
                    if matrix[state.pos.0 - 1][state.pos.1] == '#' {
                        state.dir = state.dir.turn();
                        if !visited.insert(state.clone()) {
                            is_loop = true;
                            break;
                        }
                    } else {
                        state.pos.0 -= 1;
                        if !visited.insert(state.clone()) {
                            is_loop = true;
                            break;
                        }
                    }
                } else {
                    break;
                }
            }

            Dir::Down => {
                if state.pos.0 < rows - 1 {
                    if matrix[state.pos.0 + 1][state.pos.1] == '#' {
                        state.dir = state.dir.turn();

                        if !visited.insert(state.clone()) {
                            is_loop = true;
                            break;
                        }
                    } else {
                        state.pos.0 += 1;
                        if !visited.insert(state.clone()) {
                            is_loop = true;
                            break;
                        }
                    }
                } else {
                    break;
                }
            }

            Dir::Left => {
                if state.pos.1 > 0 {
                    if matrix[state.pos.0][state.pos.1 - 1] == '#' {
                        state.dir = state.dir.turn();
                        if !visited.insert(state.clone()) {
                            is_loop = true;
                            break;
                        }
                    } else {
                        state.pos.1 -= 1;
                        if !visited.insert(state.clone()) {
                            is_loop = true;
                            break;
                        }
                    }
                } else {
                    break;
                }
            }

            Dir::Right => {
                if state.pos.1 < cols - 1 {
                    if matrix[state.pos.0][state.pos.1 + 1] == '#' {
                        state.dir = state.dir.turn();
                        if !visited.insert(state.clone()) {
                            is_loop = true;
                            break;
                        }
                    } else {
                        state.pos.1 += 1;
                        if !visited.insert(state.clone()) {
                            is_loop = true;
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }

    matrix[new_obstacle.0][new_obstacle.1] = '.';

    is_loop
}

fn part1() {
    let mut matrix = vec![];
    let mut pos = None;
    let mut dir = Dir::Null;

    for (r, line) in INPUT.lines().enumerate() {
        let mut buffer = vec![];
        for (c, ch) in line.chars().enumerate() {
            if ch == '^' {
                pos = Some((r, c));
                dir = Dir::Up;
            }
            if ch == 'V' {
                pos = Some((r, c));
                dir = Dir::Down;
            }
            if ch == '<' {
                pos = Some((r, c));
                dir = Dir::Left;
            }
            if ch == '>' {
                pos = Some((r, c));
                dir = Dir::Right
            }
            buffer.push(ch);
        }
        matrix.push(buffer);
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut pos = pos.expect("Position not found");
    let mut visited = HashSet::new();
    visited.insert(pos);

    loop {
        match dir {
            Dir::Null => break,
            Dir::Up => {
                if pos.0 > 0 {
                    if matrix[pos.0 - 1][pos.1] == '#' {
                        dir = dir.turn();
                    } else {
                        pos.0 -= 1;
                        visited.insert(pos);
                    }
                } else {
                    break;
                }
            }

            Dir::Down => {
                if pos.0 < rows - 1 {
                    if matrix[pos.0 + 1][pos.1] == '#' {
                        dir = dir.turn();
                    } else {
                        pos.0 += 1;
                        visited.insert(pos);
                    }
                } else {
                    break;
                }
            }

            Dir::Left => {
                if pos.1 > 0 {
                    if matrix[pos.0][pos.1 - 1] == '#' {
                        dir = dir.turn();
                    } else {
                        pos.1 -= 1;
                        visited.insert(pos);
                    }
                } else {
                    break;
                }
            }

            Dir::Right => {
                if pos.1 < cols - 1 {
                    if matrix[pos.0][pos.1 + 1] == '#' {
                        dir = dir.turn();
                    } else {
                        pos.1 += 1;
                        visited.insert(pos);
                    }
                } else {
                    break;
                }
            }
        }
    }

    println!("{}", visited.len());
}
