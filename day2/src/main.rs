const INPUT: &str = include_str!("../input.txt");

fn is_report_safe(report: &[u64]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for window in report.windows(2) {
        if window[0] < window[1] {
            decreasing = false;
        }
        if window[0] > window[1] {
            increasing = false;
        }

        if !increasing && !decreasing {
            return false;
        }

        let diff = window[0].abs_diff(window[1]);
        if !(1..=3).contains(&diff) {
            return false;
        }
    }

    true
}

fn part1() {
    let mut safe_count = 0;

    for line in INPUT.lines() {
        let report: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        if is_report_safe(&report) {
            safe_count += 1;
        };
    }

    println!("Safe count: {}", safe_count);
}

fn part2() {
    let mut safe_count = 0;

    for line in INPUT.lines() {
        let report: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        if is_report_safe(&report) {
            safe_count += 1;
        } else {
            for i in 0..report.len() {
                let mut report_clone = report.clone();
                report_clone.remove(i);

                if is_report_safe(&report_clone) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }

    println!("Safe count: {}", safe_count);
}

fn main() {
    part1();
    part2();
}
