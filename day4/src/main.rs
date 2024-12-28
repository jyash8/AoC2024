const INPUT: &str = include_str!("../input.txt");

#[allow(dead_code)]
fn part1() {
    let matrix: Vec<Vec<char>> = INPUT.lines().map(|s| s.chars().collect()).collect();
    let xmas: Vec<char> = "XMAS".chars().collect();

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut count = 0;
    for y in 0..rows {
        for x in 0..cols {
            if matrix[y][x] != 'X' {
                continue;
            }

            if y > 2 {
                let mut buffer = vec![];
                for i in 0..4 {
                    buffer.push(matrix[y - i][x]);
                }

                if buffer == xmas {
                    count += 1;
                }
            }

            if x > 2 {
                let mut buffer = vec![];
                for i in 0..4 {
                    buffer.push(matrix[y][x - i]);
                }

                if buffer == xmas {
                    count += 1;
                }
            }

            if y > 2 && x > 2 {
                let mut buffer = vec![];
                for i in 0..4 {
                    buffer.push(matrix[y - i][x - i]);
                }

                if buffer == xmas {
                    count += 1;
                }
            }

            if y > 2 && x < cols - 3 {
                let mut buffer = vec![];
                for i in 0..4 {
                    buffer.push(matrix[y - i][x + i]);
                }

                if buffer == xmas {
                    count += 1;
                }
            }

            if x > 2 && y < rows - 3 {
                let mut buffer = vec![];
                for i in 0..4 {
                    buffer.push(matrix[y + i][x - i]);
                }

                if buffer == xmas {
                    count += 1;
                }
            }

            if y < rows - 3 {
                let mut buffer = vec![];
                for i in 0..4 {
                    buffer.push(matrix[y + i][x]);
                }

                if buffer == xmas {
                    count += 1;
                }
            }

            if x < cols - 3 {
                let mut buffer = vec![];
                for i in 0..4 {
                    buffer.push(matrix[y][x + i]);
                }

                if buffer == xmas {
                    count += 1;
                }
            }

            if y < rows - 3 && x < cols - 3 {
                let mut buffer = vec![];
                for i in 0..4 {
                    buffer.push(matrix[y + i][x + i]);
                }

                if buffer == xmas {
                    count += 1;
                }
            }
        }
    }

    println!("Count: {}", count);
}

fn part2() {
    let matrix: Vec<Vec<char>> = INPUT.lines().map(|s| s.chars().collect()).collect();

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut count = 0;

    for y in 1..rows - 1 {
        for x in 1..cols - 1 {
            if matrix[y][x] != 'A' {
                continue;
            }

            if ((matrix[y - 1][x - 1] == 'M' && matrix[y + 1][x + 1] == 'S')
                || (matrix[y - 1][x - 1] == 'S' && matrix[y + 1][x + 1] == 'M'))
                && ((matrix[y - 1][x + 1] == 'M' && matrix[y + 1][x - 1] == 'S')
                    || matrix[y - 1][x + 1] == 'S' && matrix[y + 1][x - 1] == 'M')
            {
                count += 1;
            }
        }
    }

    println!("Count: {}", count);
}

fn main() {
    part1();
    part2();
}
