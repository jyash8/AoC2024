const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut line_split = INPUT.split("\n\n");

    // Not needed for part2
    let mut _registers: Vec<usize> = line_split
        .next()
        .unwrap()
        .lines()
        .map(|line| line[12..].parse().unwrap())
        .collect();
    let instructions: Vec<usize> = line_split.next().unwrap()[9..]
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    if false {
        let mut output = vec![];
        let mut pointer = 0;
        while pointer < instructions.len() - 1 {
            let c_operand = match instructions[pointer + 1] {
                a @ 0..=3 => format!("{}", a),
                4 => "a".to_string(),
                5 => "b".to_string(),
                6 => "c".to_string(),
                _ => "d".to_string(),
            };

            let instruction = match instructions[pointer] {
                0 => format!("a = a >> {}", c_operand),
                1 => format!("b = b ^ {}", instructions[pointer + 1]),
                2 => format!("b = {} % 8", c_operand),
                3 => format!("if a!=0 {{ pc = {} }}", instructions[pointer + 1]),
                4 => "b = b ^ c".to_string(),
                5 => format!("out({} % 8)", c_operand),
                6 => format!("b = a >> {}", c_operand),
                7 => format!("c = a >> {}", c_operand),
                d => unreachable!("{d}"),
            };

            output.push(instruction);

            pointer += 2;
        }

        for code in output {
            println!("{}", code);
        }
    } else {
        println!("{:?}", find(&instructions, 0));
    }
}

fn find(instructions: &[usize], ans: usize) -> Option<usize> {
    println!("{:?} {}", instructions, ans);
    if instructions.is_empty() {
        return Some(ans);
    }
    let mut a;
    let mut c;
    for mut b in 0..8 {
        a = (ans << 3) + b;
        b ^= 1;
        c = a >> b;
        b ^= 5;
        b ^= c;
        if b % 8 == *instructions.last().unwrap() {
            let len = instructions.len();
            let sub = find(&instructions[..len - 1], a);
            if let Some(sol) = sub {
                return Some(sol);
            } else {
                continue;
            }
        }
    }
    None
}

fn part1() {
    let mut line_split = INPUT.split("\n\n");

    let mut registers: Vec<usize> = line_split
        .next()
        .unwrap()
        .lines()
        .map(|line| line[12..].parse().unwrap())
        .collect();
    let instructions: Vec<usize> = line_split.next().unwrap()[9..]
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let mut output = vec![];

    let mut pointer = 0;

    while pointer < instructions.len() - 1 {
        let c_operand = match instructions[pointer + 1] {
            a @ 0..=3 => a,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            d => d,
        };

        match instructions[pointer] {
            0 => registers[0] >>= c_operand,
            1 => registers[1] ^= instructions[pointer + 1],
            2 => registers[1] = c_operand % 8,
            3 => {
                if registers[0] != 0 {
                    pointer = instructions[pointer + 1];
                    continue;
                }
            }
            4 => registers[1] ^= registers[2],
            5 => output.push(c_operand % 8),
            6 => registers[1] = registers[0] >> c_operand,
            7 => registers[2] = registers[0] >> c_operand,
            d => unreachable!("{d}"),
        }

        pointer += 2;
    }

    let mut result_string = format!("{}", output[0]);
    for num in &output[1..] {
        result_string += &format!(",{}", num);
    }

    println!("{}", result_string);
}
