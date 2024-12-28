use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut newline_split = INPUT.split("\n\n");

    let mut rule_map: HashMap<u64, HashSet<u64>> = HashMap::new();

    let rules = newline_split.next().unwrap();
    let reports = newline_split.next().unwrap();

    for rule in rules.lines() {
        let nums: Vec<u64> = rule.split('|').map(|s| s.parse().unwrap()).collect();

        rule_map.entry(nums[1]).or_default().insert(nums[0]);
    }

    let report_partition: (Vec<Vec<u64>>, Vec<Vec<u64>>) = reports
        .lines()
        .map(|raw_report| {
            raw_report
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u64>>()
        })
        .partition(|parsed_report| is_accepted(parsed_report, &rule_map));

    let sum_of_middle_num_correct = report_partition
        .0
        .into_iter()
        .map(|accepted_report| accepted_report[accepted_report.len() / 2])
        .sum::<u64>();

    let sum_fixed_report_middle_num = report_partition
        .1
        .into_iter()
        .map(|mut report| {
            let n = report.len();
            for i in 0..n {
                let mut swapped = false;
                for j in 0..(n - 1 - i) {
                    if rule_map.get(&report[j]).unwrap().contains(&report[j + 1]) {
                        // Swap the elements
                        report.swap(j, j + 1);
                        swapped = true;
                    }
                }
                if !swapped {
                    break;
                }
            }

            report
        })
        .map(|accepted_report| accepted_report[accepted_report.len() / 2])
        .sum::<u64>();

    println!("Part1: {}", sum_of_middle_num_correct);
    println!("Part2: {}", sum_fixed_report_middle_num);
}

fn is_accepted(report: &[u64], rule_map: &HashMap<u64, HashSet<u64>>) -> bool {
    for (index, num) in report[..report.len() - 1].iter().enumerate() {
        let entry_prev = report[index + 1..].iter().any(|n| match rule_map.get(num) {
            Some(entry) => entry.contains(n),
            None => false,
        });

        if entry_prev {
            return false;
        }
    }

    true
}
