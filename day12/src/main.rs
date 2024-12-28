use std::{collections::HashSet, fmt::Debug, hash::Hash};

use rayon::{iter::ParallelIterator, str::ParallelString};

const INPUT: &str = include_str!("../test.txt");

#[derive(Clone, PartialEq, PartialOrd)]
struct Loc(f32, f32);

impl Debug for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Hash for Loc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
        self.1.to_bits().hash(state);
    }
}

impl Eq for Loc {}

fn main() {
    let matrix: Vec<Vec<char>> = INPUT
        .par_lines()
        .map(|line| line.par_chars().collect())
        .collect();

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut seen = HashSet::new();
    let mut regions = vec![];

    for y in 0..rows {
        for x in 0..cols {
            if seen.contains(&(y, x)) {
                continue;
            }

            let mut region = HashSet::new();
            let mut stack = vec![(y, x)];

            while let Some(loc) = stack.pop() {
                if seen.contains(&loc) {
                    continue;
                }

                region.insert(loc);
                seen.insert(loc);
                stack.extend(next_positions(&matrix, &seen, loc));
            }

            regions.push((matrix[y][x], region));
        }
    }

    let cost = regions
        .iter()
        .map(|(_ch, region)| {
            let area = region.len();

            let perimeter: usize = region
                .iter()
                .map(|(y, x)| {
                    let mut side_count = 0;
                    if let Some(ch) = matrix.get_i(*y as isize - 1, *x as isize) {
                        if ch != &matrix[*y][*x] {
                            side_count += 1;
                        }
                    } else {
                        side_count += 1;
                    }
                    if let Some(ch) = matrix.get_i(*y as isize + 1, *x as isize) {
                        if ch != &matrix[*y][*x] {
                            side_count += 1;
                        }
                    } else {
                        side_count += 1;
                    }
                    if let Some(ch) = matrix.get_i(*y as isize, *x as isize - 1) {
                        if ch != &matrix[*y][*x] {
                            side_count += 1;
                        }
                    } else {
                        side_count += 1;
                    }
                    if let Some(ch) = matrix.get_i(*y as isize, *x as isize + 1) {
                        if ch != &matrix[*y][*x] {
                            side_count += 1;
                        }
                    } else {
                        side_count += 1;
                    }

                    side_count
                })
                .sum();

            let sides = sides_func(
                region
                    .iter()
                    .map(|c| (c.0 as isize, c.1 as isize))
                    .collect(),
            );

            (area * perimeter, sides * area)
        })
        .fold((0usize, 0usize), |(mut acc1, mut acc2), (c1, c2)| {
            acc1 += c1;
            acc2 += c2;
            (acc1, acc2)
        });

    println!("Part1 {:?}", cost.0);
    println!("Part2 {:?}", cost.1);
}

fn next_positions(
    matrix: &[Vec<char>],
    seen: &HashSet<(usize, usize)>,
    (y, x): (usize, usize),
) -> Vec<(usize, usize)> {
    let mut result = vec![];

    if x > 0 && !seen.contains(&(y, x - 1)) && matrix[y][x] == matrix[y][x - 1] {
        result.push((y, x - 1));
    }

    if y > 0 && !seen.contains(&(y - 1, x)) && matrix[y][x] == matrix[y - 1][x] {
        result.push((y - 1, x));
    }

    if x < matrix[0].len() - 1 && !seen.contains(&(y, x + 1)) && matrix[y][x] == matrix[y][x + 1] {
        result.push((y, x + 1));
    }

    if y < matrix.len() - 1 && !seen.contains(&(y + 1, x)) && matrix[y][x] == matrix[y + 1][x] {
        result.push((y + 1, x));
    }

    result
}

fn sides_func(region: HashSet<(isize, isize)>) -> usize {
    let mut corner_candidates = HashSet::new();

    for (r, c) in region.iter().map(|(r, c)| (*r as f32, *c as f32)) {
        for (cr, cc) in [
            (r - 0.5, c - 0.5),
            (r + 0.5, c - 0.5),
            (r + 0.5, c + 0.5),
            (r - 0.5, c + 0.5),
        ] {
            corner_candidates.insert(Loc(cr, cc));
        }
    }
    let mut corners = 0;

    for Loc(cr, cc) in corner_candidates {
        let config = [
            (cr - 0.5, cc - 0.5),
            (cr + 0.5, cc - 0.5),
            (cr + 0.5, cc + 0.5),
            (cr - 0.5, cc + 0.5),
        ]
        .map(|(sr, sc)| region.contains(&(sr as isize, sc as isize)));

        let number = config.map(|boolean| boolean as i32).iter().sum::<i32>();

        if number == 1 {
            corners += 1;
        } else if number == 2 {
            if config == [true, false, true, false] || config == [false, true, false, true] {
                corners += 2;
            }
        } else if number == 3 {
            corners += 1
        }
        // case for 0 ignored as impossible case for 4 ignored as corners will be zero
    }

    corners
}

trait GetByIsize<T> {
    fn get_i(&self, y: isize, x: isize) -> Option<&T>;
}

impl<T> GetByIsize<T> for Vec<Vec<T>> {
    fn get_i(&self, y: isize, x: isize) -> Option<&T> {
        if y < 0 || x < 0 {
            return None;
        }

        self.get(y as usize).and_then(|v| v.get(x as usize))
    }
}
