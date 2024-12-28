use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, PartialEq)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
struct Expr<'a> {
    op1: &'a str,
    op2: &'a str,
    op: Op,
}

#[allow(dead_code)]
fn pp<'a>(wire: &'a str, depth: usize, expr_map: &HashMap<&str, Expr<'a>>) -> String {
    if [b'x', b'y'].contains(&wire.as_bytes()[0]) {
        return format!("{:>width$}", wire, width = 2 * depth + wire.len());
    }

    let expr = expr_map[wire].clone();
    let op = format!("{:?}", expr.op);
    format!(
        "{:>width$} ({wire})\n{ppx}\n{ppy}",
        op,
        width = 2 * depth + op.len(),
        ppx = pp(expr.op1, depth + 1, expr_map),
        ppy = pp(expr.op2, depth + 1, expr_map)
    )
}

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut line_split = INPUT.split("\n\n");

    let mut expr_map: HashMap<&str, Expr> = line_split
        .nth(1)
        .unwrap()
        .lines()
        .map(|line| {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            (
                tokens[4],
                Expr {
                    op1: tokens[0],
                    op2: tokens[2],
                    op: match tokens[1] {
                        "AND" => Op::And,
                        "OR" => Op::Or,
                        "XOR" => Op::Xor,
                        _ => unreachable!(),
                    },
                },
            )
        })
        .collect();

    let mut wires = vec![];

    for _ in 0..5 {
        let baseline = progress(&expr_map);
        let map_clone = expr_map.clone();
        for comb in map_clone.keys().combinations(2) {
            unsafe {
                std::ptr::swap(
                    expr_map.get_mut(comb[0]).unwrap() as *mut Expr,
                    expr_map.get_mut(comb[1]).unwrap() as *mut Expr,
                );
            }

            if progress(&expr_map) > baseline {
                wires.extend(comb.into_iter().map(|s| s.to_string()));
                break;
            }

            unsafe {
                std::ptr::swap(
                    expr_map.get_mut(comb[0]).unwrap() as *mut Expr,
                    expr_map.get_mut(comb[1]).unwrap() as *mut Expr,
                );
            }
        }
    }

    wires.sort();
    println!("{}", wires.into_iter().join(","));
}

fn progress(expr_map: &HashMap<&str, Expr>) -> usize {
    let mut i = 0;

    loop {
        if !verify(i, expr_map) {
            break;
        }
        i += 1;
    }
    i
}

fn verify_z(wire: &str, num: usize, expr_map: &HashMap<&str, Expr>) -> bool {
    // println!("vz {} {}", wire, num);

    if !expr_map.contains_key(wire) {
        return false;
    }
    let Expr { op1: x, op2: y, op } = expr_map[wire].clone();
    if op != Op::Xor {
        return false;
    }

    if num == 0 {
        let mut ops = [x, y];
        ops.sort();
        return ops == ["x00", "y00"];
    }

    verify_inter_xor(x, num, expr_map) && verify_carry_bit(y, num, expr_map)
        || verify_inter_xor(y, num, expr_map) && verify_carry_bit(x, num, expr_map)
}

fn make_wire(ch: char, num: usize) -> String {
    format!("{ch}{:0>2}", num)
}

fn verify_inter_xor(wire: &str, num: usize, expr_map: &HashMap<&str, Expr>) -> bool {
    // println!("vx {} {}", wire, num);
    if !expr_map.contains_key(wire) {
        return false;
    }
    let Expr { op1: x, op2: y, op } = expr_map[wire].clone();
    if op != Op::Xor {
        return false;
    }

    let mut temp = [x, y];
    temp.sort();
    temp == [make_wire('x', num), make_wire('y', num)]
}

fn verify_carry_bit(wire: &str, num: usize, expr_map: &HashMap<&str, Expr>) -> bool {
    // println!("vc {} {}", wire, num);
    if !expr_map.contains_key(wire) {
        return false;
    }
    let Expr { op1: x, op2: y, op } = expr_map[wire].clone();
    if num == 1 {
        let mut temp = [x, y];
        temp.sort();
        return op == Op::And && x == "x00" && y == "y00";
    }

    if op != Op::Or {
        return false;
    }
    verify_direct_carry(x, num - 1, expr_map) && verify_recarry(y, num - 1, expr_map)
        || verify_direct_carry(y, num - 1, expr_map) && verify_recarry(x, num - 1, expr_map)
}

fn verify_direct_carry(wire: &str, num: usize, expr_map: &HashMap<&str, Expr>) -> bool {
    // println!("vd {} {}", wire, num);
    if !expr_map.contains_key(wire) {
        return false;
    }
    let Expr { op1: x, op2: y, op } = expr_map[wire].clone();

    if op != Op::And {
        return false;
    }
    let mut temp = [x, y];
    temp.sort();

    temp == [make_wire('x', num), make_wire('y', num)]
}

fn verify_recarry(wire: &str, num: usize, expr_map: &HashMap<&str, Expr>) -> bool {
    // println!("vr {} {}", wire, num);
    if !expr_map.contains_key(wire) {
        return false;
    }
    let Expr { op1: x, op2: y, op } = expr_map[wire].clone();

    if op != Op::And {
        return false;
    }

    verify_inter_xor(x, num, expr_map) && verify_carry_bit(y, num, expr_map)
        || verify_inter_xor(y, num, expr_map) && verify_carry_bit(x, num, expr_map)
}

fn verify(num: usize, expr_map: &HashMap<&str, Expr>) -> bool {
    verify_z(&make_wire('z', num), num, expr_map)
}

fn part1() {
    let mut line_split = INPUT.split("\n\n");

    let mut value_map: HashMap<&str, u8> = line_split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut colon_split = line.split(": ");
            let node = colon_split.next().unwrap();
            let val = colon_split.next().unwrap().parse::<u8>().unwrap();
            (node, val)
        })
        .collect();

    let expr_map: HashMap<&str, Expr> = line_split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            (
                tokens[4],
                Expr {
                    op1: tokens[0],
                    op2: tokens[2],
                    op: match tokens[1] {
                        "AND" => Op::And,
                        "OR" => Op::Or,
                        "XOR" => Op::Xor,
                        _ => unreachable!(),
                    },
                },
            )
        })
        .collect();

    for expr in expr_map.iter() {
        eval((*expr.0, expr.1.clone()), &expr_map, &mut value_map);
    }

    let mut z_value: Vec<_> = value_map
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .collect();
    z_value.sort();

    let mut number = 0u64;
    for (index, (_, &val)) in z_value.into_iter().enumerate() {
        if val == 1 {
            number |= 1 << index;
        }
    }

    println!("{:?}", number);
}

fn eval<'a>(
    expr: (&'a str, Expr<'a>),
    expr_map: &HashMap<&str, Expr<'a>>,
    value_map: &mut HashMap<&'a str, u8>,
) {
    if !value_map.contains_key(expr.1.op1) {
        eval(
            (expr.1.op1, expr_map[expr.1.op1].clone()),
            expr_map,
            value_map,
        );
    }

    if !value_map.contains_key(expr.1.op2) {
        eval(
            (expr.1.op2, expr_map[expr.1.op2].clone()),
            expr_map,
            value_map,
        );
    }

    let op1 = value_map[expr.1.op1];
    let op2 = value_map[expr.1.op2];

    let result = match expr.1.op {
        Op::And => op1 & op2,
        Op::Or => op1 | op2,
        Op::Xor => op1 ^ op2,
    };

    value_map.insert(expr.0, result);
}
