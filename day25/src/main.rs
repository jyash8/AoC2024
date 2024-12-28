use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut locks = vec![];
    let mut keys = vec![];

    for chunk in INPUT.split("\n\n") {
        let matrix: Vec<Vec<char>> = chunk.lines().map(|line| line.chars().collect()).collect();
        // let rows = matrix.len();
        let cols = matrix[0].len();

        if matrix[0].iter().all(|&ch| ch == '#') {
            let mut buffer = vec![];
            for i in 0..cols {
                let mut count = 0;
                for row in &matrix[1..] {
                    if row[i] != '#' {
                        break;
                    }
                    count += 1;
                }
                buffer.push(count);
            }
            locks.push(buffer);
        }

        if matrix.last().unwrap().iter().all(|&ch| ch == '#') {
            // it is a key
            let mut buffer = vec![];
            for i in 0..cols {
                let mut count = 0;
                for row in matrix[0..matrix.len() - 1].iter().rev() {
                    if row[i] != '#' {
                        break;
                    }
                    count += 1;
                }
                buffer.push(count);
            }
            keys.push(buffer);
        }
    }

    let count = locks
        .into_iter()
        .cartesian_product(keys)
        .filter(|product| {
            product
                .0
                .iter()
                .zip(product.1.iter())
                .map(|(num1, num2)| num1 + num2)
                .all(|sum| sum < 6)
        })
        .count();

    println!("{}", count);
}
