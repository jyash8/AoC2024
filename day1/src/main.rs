use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn part1() {
    let mut list1 = vec![];
    let mut list2 = vec![];

    for line in INPUT.lines() {
        let mut nums = line.split_whitespace();
        list1.push(nums.next().unwrap().parse::<i64>().unwrap());
        list2.push(nums.next().unwrap().parse::<i64>().unwrap());
    }

    list1.sort();
    list2.sort();

    let sum: u64 = list1
        .into_iter()
        .zip(list2)
        .map(|(n1, n2)| n1.abs_diff(n2))
        .sum();

    println!("Part1: {}", sum);
}

fn part2() {
    let mut counter = HashMap::new();
    let mut list = vec![];

    for line in INPUT.lines() {
        let mut nums = line.split_whitespace();

        list.push(nums.next().unwrap().parse::<i64>().unwrap());
        *counter
            .entry(nums.next().unwrap().parse::<i64>().unwrap())
            .or_insert(0) += 1;
    }

    let sum: i64 = list
        .iter()
        .map(|num| counter.get(num).unwrap_or(&0) * num)
        .sum();

    println!("Part2: {}", sum);
}

fn main() {
    part1();
    part2();
}
