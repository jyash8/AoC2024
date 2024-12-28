use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

fn part2() {
    let edges = INPUT
        .lines()
        .map(|line| line.split('-').collect::<Vec<_>>());

    let mut conns: HashMap<&str, HashSet<&str>> = HashMap::new();

    for edge in edges {
        conns.entry(edge[0]).or_default().insert(edge[1]);
        conns.entry(edge[1]).or_default().insert(edge[0]);
    }

    let mut sets = HashSet::new();
    for x in conns.keys() {
        let mut req = HashSet::new();
        req.insert(*x);
        search(x, req, &conns, &mut sets);
    }

    println!(
        "{:?}",
        sets.into_iter()
            .max_by_key(|v| v.len())
            .map(|st| st.join(","))
    );
}

fn search<'a>(
    node: &str,
    req: HashSet<&'a str>,
    conns: &HashMap<&str, HashSet<&'a str>>,
    sets: &mut HashSet<Vec<&'a str>>,
) {
    let mut key: Vec<&str> = req.iter().cloned().collect();
    key.sort();
    if !sets.insert(key) {
        return;
    }

    for neighbor in &conns[node] {
        if req.contains(neighbor) {
            continue;
        }

        if !req.iter().all(|query| conns[query].contains(neighbor)) {
            // if !req.is_subset(&conns[neighbor]) {
            continue;
        }

        let mut new_req = req.clone();
        new_req.insert(neighbor);

        search(neighbor, new_req, conns, sets);
    }
}

fn part1() {
    let edges = INPUT
        .lines()
        .map(|line| line.split('-').collect::<Vec<_>>());

    let mut conns: HashMap<&str, HashSet<&str>> = HashMap::new();

    for edge in edges {
        conns.entry(edge[0]).or_default().insert(edge[1]);
        conns.entry(edge[1]).or_default().insert(edge[0]);
    }

    let mut sets = HashSet::new();

    for (x, conn) in conns.iter() {
        for y in conn {
            for z in conns[y].iter() {
                if x != z && conns[z].contains(x) {
                    let mut temp = [x, y, z];
                    temp.sort();
                    sets.insert(temp);
                }
            }
        }
    }

    let final_count = sets
        .iter()
        .filter(|set| set.iter().any(|node| node.starts_with('t')))
        .count();

    println!("{}", final_count);
}
