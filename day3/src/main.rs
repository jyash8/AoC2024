use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn part1() {
    let pattern = r"mul\([0-9]*,[0-9]*\)";

    let regex = Regex::new(pattern).unwrap();

    let sum = regex
        .captures_iter(INPUT)
        .map(|capture| {
            let mut slice = String::from(&capture[0][4..]);
            slice.pop();

            slice
                .split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .product::<u64>()
        })
        .sum::<u64>();

    println!("Sum: {}", sum);
}

fn part2() {
    let pattern = r"(mul\([0-9]*,[0-9]*\)|do\(\)|don't\(\))";

    let regex = Regex::new(pattern).unwrap();

    let mut sum = 0;
    let mut enabled = true;
    for capture in regex.captures_iter(INPUT) {
        match capture[0].get(0..3) {
            Some("do(") => enabled = true,
            Some("don") => enabled = false,
            Some("mul") => {
                if enabled {
                    let mut slice = String::from(&capture[0][4..]);
                    slice.pop();
                    sum += slice
                        .split(',')
                        .map(|s| s.parse::<u64>().unwrap())
                        .product::<u64>();
                }
            }
            _ => unreachable!(),
        };
    }

    println!("Sum: {}", sum);
}

fn main() {
    part1();
    part2();
}
