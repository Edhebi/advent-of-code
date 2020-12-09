#![feature(str_split_once)]

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    character: char,
}

fn parse_line(line: &str) -> (Policy, &str) {
    let (prefix, pass) = line.split_once(':').unwrap();
    let (min_max, character) = prefix.split_once(' ').unwrap();
    let (min, max) = min_max.split_once('-').unwrap();
    let policy = Policy {
        min: min.parse().unwrap(),
        max: max.parse().unwrap(),
        character: character.chars().nth(0).unwrap(),
    };
    (policy, pass.trim())
}

fn common(matcher: impl Fn(&Policy, &str) -> bool) -> usize {
    include_str!("../inputs/day02")
        .lines()
        .map(parse_line)
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
