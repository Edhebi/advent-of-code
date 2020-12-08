#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    character: char,
}

fn parse_line(line: &str) -> Option<(Policy, &str)> {
    let segments: Vec<&str> = line.split(' ').collect();
    let minmax: Vec<&str> = segments.get(0)?.split('-').collect();
    let policy = Policy {
        min: minmax.get(0)?.parse().ok()?,
        max: minmax.get(1)?.parse().ok()?,
        character: segments.get(1)?.chars().nth(0)?,
    };
    Some((policy, *segments.get(2)?))
}

fn common(matcher: impl Fn(&Policy, &str) -> bool) -> usize {
    include_str!("../inputs/day02")
        .lines()
        .filter_map(parse_line)
        .filter(|pair| matcher(&pair.0, pair.1))
        .count()
}

fn part1() -> usize {
    common(|policy: &Policy, pass: &str| {
        let n = pass.chars().filter(|&c| c == policy.character).count();
        n >= policy.min && n <= policy.max
    })
}

fn part2() -> usize {
    common(|policy: &Policy, pass: &str| {
        [policy.min, policy.max]
            .iter()
            .filter_map(|&i| pass.chars().nth(i - 1))
            .filter(|&c| c == policy.character)
            .count()
            == 1
    })
}

fn main() {
    println!("Day 2:");
    println!("1: {}", part1());
    println!("2: {}", part2());
}
