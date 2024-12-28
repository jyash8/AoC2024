use std::usize;

use regex::Regex;

// const INPUT: &str = include_str!("../test.txt");
// const HEIGHT: i64 = 7;
// const WIDTH: i64 = 11;

const INPUT: &str = include_str!("../input.txt");
const HEIGHT: i64 = 103;
const WIDTH: i64 = 101;

#[derive(Debug)]
struct Robot {
    pos: (i64, i64),
    vel: (i64, i64),
}

fn modulo(a: i64, b: i64) -> i64 {
    let div = a / b;
    let result = a - (div * b);
    if result < 0 {
        b + result
    } else {
        result
    }
}

impl Robot {
    fn steps(&mut self, steps: i64) {
        self.pos.0 = modulo(self.pos.0 + self.vel.0 * steps, WIDTH);
        self.pos.1 = modulo(self.pos.1 + self.vel.1 * steps, HEIGHT);
    }
}

fn main() {
    part1();
    part2();
}

fn part2() {
    let hm = (WIDTH - 1) / 2;
    let vm = (HEIGHT - 1) / 2;

    let pattern = r"-?\d+";
    let regex = Regex::new(pattern).unwrap();

    let robots: Vec<_> = INPUT
        .lines()
        .map(|line| {
            let nums: Vec<i64> = regex
                .captures_iter(line)
                .map(|cap_grp| cap_grp[0].parse().unwrap())
                .collect();
            Robot {
                pos: (nums[0], nums[1]),
                vel: (nums[2], nums[3]),
            }
        })
        .collect();

    let mut min_sf = usize::MAX;
    let mut best_iteration = 0;

    for second in 0..WIDTH * HEIGHT {
        let mut result = vec![];

        for Robot {
            pos: (px, py),
            vel: (vx, vy),
        } in robots.iter()
        {
            result.push((
                modulo(px + vx * second, WIDTH),
                modulo(py + vy * second, HEIGHT),
            ));
        }

        let mut tl = 0;
        let mut bl = 0;
        let mut tr = 0;
        let mut br = 0;

        for &(px, py) in result.iter() {
            if px == hm || py == vm {
                continue;
            }

            if px < hm {
                if py < vm {
                    tl += 1;
                } else {
                    bl += 1;
                }
            } else if py < vm {
                tr += 1;
            } else {
                br += 1;
            }
        }

        let sf = tl * bl * tr * br;

        if sf < min_sf {
            min_sf = sf;
            best_iteration = second;
        }
    }

    println!("{} {}", min_sf, best_iteration);
}

fn part1() {
    let hm = (WIDTH - 1) / 2;
    let vm = (HEIGHT - 1) / 2;

    let pattern = r"-?\d+";
    let regex = Regex::new(pattern).unwrap();

    let robots: Vec<_> = INPUT
        .lines()
        .map(|line| {
            let nums: Vec<i64> = regex
                .captures_iter(line)
                .map(|cap_grp| cap_grp[0].parse().unwrap())
                .collect();
            let mut robot = Robot {
                pos: (nums[0], nums[1]),
                vel: (nums[2], nums[3]),
            };

            robot.steps(100);
            robot
        })
        .collect();

    let mut quad = [0usize; 4];

    for Robot {
        pos: (px, py),
        vel: _,
    } in robots
    {
        if px == hm || py == vm {
            continue;
        }

        if px < hm {
            if py < vm {
                quad[0] += 1
            } else {
                quad[1] += 1
            }
        } else if py < vm {
            quad[2] += 1
        } else {
            quad[3] += 1
        }
    }

    println!("{:?}", quad);
    println!("{:?}", quad.iter().product::<usize>());
}
