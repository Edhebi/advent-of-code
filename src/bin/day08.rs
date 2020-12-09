#[derive(Copy, Clone, Debug)]
enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Op {
    fn flip(&mut self) {
        *self = match *self {
            Self::Nop(n) => Self::Jmp(n),
            Self::Jmp(n) => Self::Nop(n),
            Self::Acc(n) => Self::Acc(n),
        }
    }

    fn can_flip(&self) -> bool {
        match *self {
            Self::Acc(_) => false,
            _ => true,
        }
    }

    fn from_str(line: &str) -> Self {
        let (op, acc) = line.split_at(3);
        let acc = acc.trim().parse().unwrap();
        match op {
            "nop" => Op::Nop(acc),
            "jmp" => Op::Jmp(acc),
            "acc" => Op::Acc(acc),
            _ => panic!("unknown op: {}", op),
        }
    }
}

fn parse(input: &str) -> Vec<Op> {
    input.lines().map(Op::from_str).collect()
}

fn exec(ops: &[Op]) -> (bool, i32) {
    let mut visited = vec![false; ops.len()];
    let mut sp = 0;
    let mut acc = 0;
    while !visited[sp] {
        visited[sp] = true;
        match ops[sp] {
            Op::Nop(_) => sp += 1,
            Op::Jmp(n) => sp = (sp as isize + n as isize) as usize,
            Op::Acc(n) => {
                sp += 1;
                acc += n
            }
        }
        if sp >= ops.len() {
            return (true, acc);
        }
    }
    (false, acc)
}

fn find_patch(mut ops: Vec<Op>) -> i32 {
    for i in 0..ops.len() {
        if !ops[i].can_flip() {
            continue;
        }
        (&mut ops[i]).flip();
        if let (true, n) = exec(&ops) {
            return n;
        }
        (&mut ops[i]).flip();
    }
    panic!("no patching found");
}

fn part1() -> i32 {
    let input = include_str!("../inputs/day08");
    let ops = parse(input);
    exec(&ops).1
}

fn part2() -> i32 {
    let input = include_str!("../inputs/day08");
    let ops = parse(input);
    find_patch(ops)
}

fn main() {
    println!("Day 8:");
    println!("1: {}", part1());
    println!("2: {}", part2());
}
