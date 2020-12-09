use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn parse_line(line: &str) -> (String, Vec<(usize, String)>) {
    const KEY_STR: &str = r"^(?P<color>\w+ \w+)";
    const VAL_RE_STR: &str = r"(?P<amount>\d+) (?P<color>\w+ \w+) bags?";
    lazy_static! {
        static ref KEY_RE: Regex = Regex::new(KEY_STR).unwrap();
        static ref VAL_RE: Regex = Regex::new(VAL_RE_STR).unwrap();
    }
    let key = Regex::captures(&KEY_RE, line).unwrap()["color"].into();
    let values = Regex::captures_iter(&VAL_RE, line)
        .map(|c| (c["amount"].parse().unwrap(), c["color"].into()))
        .collect();
    (key, values)
}

fn reverse_map(input: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for (parent, children) in input.lines().map(parse_line) {
        for (_, child) in children {
            map.entry(child).or_insert(Vec::new()).push(parent.clone());
        }
    }
    map
}

fn part1() -> usize {
    let input = include_str!("../inputs/day07");
    let map = reverse_map(input);

    let mut to_visit = vec!["shiny gold"];
    let mut visited = HashSet::new();
    while !to_visit.is_empty() {
        let key = to_visit.pop().unwrap();
        if visited.contains(key) {
            continue;
        }
        visited.insert(key);
        map.get(key)
            .map(|v| v.iter().for_each(|p| to_visit.push(p)));
    }

    visited.len() - 1
}

fn part2() -> usize {
    let input = include_str!("../inputs/day07");
    let map: HashMap<_, _> = input.lines().map(parse_line).collect();

    let mut stack = vec!["shiny gold"];
    let mut amount: HashMap<&str, usize> = Default::default();
    while !stack.is_empty() {
        let &parent = stack.last().unwrap();
        let children = map.get(parent).unwrap();
        if children.is_empty() {
            amount.insert(parent, 0);
            stack.pop();
            continue;
        }
        if children
            .iter()
            .all(|(_, child)| amount.contains_key(&child[..]))
        {
            let total = children
                .iter()
                .map(|(count, color)| count * (1 + amount[&color[..]]))
                .sum();
            amount.insert(parent, total);
            stack.pop();
            continue;
        }
        children
            .iter()
            .filter(|(_, child)| !amount.contains_key(&child[..]))
            .for_each(|(_, child)| stack.push(&child[..]));
    }
    amount["shiny gold"]
}

fn main() {
    println!("Day 7:");
    println!("1: {}", part1());
    println!("2: {}", part2());
}
