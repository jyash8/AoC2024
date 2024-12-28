const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

fn part1() {
    let solutions: f64 = INPUT
        .trim()
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .flat_map(|line| {
                    line.split(':')
                        .nth(1)
                        .unwrap()
                        .split(',')
                        .map(|s| s[3..].parse::<f64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|nums| {
            solve(
                nums[0],
                nums[2],
                nums[1],
                nums[3],
                nums[4],
                nums[5],
            )
        })
        .filter_map(|sol| match sol {
            Solution::None => None,
            Solution::Unique((x, y)) => {
                if x.fract() != 0.0 || y.fract() != 0.0 {
                    None
                } else {
                    Some(3.0 * x + y)
                }
            }
            Solution::Many => unimplemented!(),
        })
        .sum();

    println!("{:?}", solutions as u64);
}

fn part2() {
    let solutions: f64 = INPUT
    .trim()
    .split("\n\n")
    .map(|chunk| {
        chunk
        .lines()
        .flat_map(|line| {
            line.split(':')
            .nth(1)
            .unwrap()
            .split(',')
            .map(|s| s[3..].parse::<f64>().unwrap())
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
    })
    .map(|nums| {
        solve(
            nums[0],
            nums[2],
            nums[1],
            nums[3],
            nums[4] + 10000000000000.0,
            nums[5] + 10000000000000.0,
        )
    })
    .filter_map(|sol| match sol {
        Solution::None => None,
        Solution::Unique((x, y)) => {
            if x.fract() != 0.0 || y.fract() != 0.0 {
                None
            } else {
                Some(3.0 * x + y)
            }
        }
        Solution::Many => unimplemented!(),
    })
    .sum();

    println!("{:?}", solutions as u64);
}

#[derive(Debug, Clone)]
enum Solution {
    Unique((f64, f64)),
    Many,
    None,
}

fn solve(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Solution {
    // println!("{}x + {}y = {}", a, b, e);
    // println!("{}x + {}y = {}\n", c, d, f);

    let det = a * d - b * c;

    if det == 0.0 {
        if a * f == e * c && b * f == d * e {
            return Solution::Many;
        } else {
            return Solution::None;
        }
    }

    Solution::Unique(((d * e - b * f) / det, (a * f - c * e) / det))
}
