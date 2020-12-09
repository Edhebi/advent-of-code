use std::collections::HashSet;

fn sum2(entries: &HashSet<i32>, target: i32) -> Option<(i32, i32)> {
    entries
        .iter()
        .filter(|&a| *a <= target)
        .map(|&a| (a, target - a))
        .find(|&(_a, b)| entries.contains(&b))
}

fn sum3(entries: &HashSet<i32>, target: i32) -> Option<(i32, i32, i32)> {
    entries
        .iter()
        .filter_map(|&a| sum2(entries, target - a).map(|(b, c)| (a, b, c)))
        .find(|&(a, b, c)| a != b && a != c && b != c)
}

fn entries() -> HashSet<i32> {
    let input = include_str!("../inputs/day01");
    input.lines().map(|s| s.parse().unwrap()).collect()
}

fn part1() -> i32 {
    let (a, b) = sum2(&entries(), 2020).unwrap();
    a * b
}
fn part2() -> i32 {
    let (a, b, c) = sum3(&entries(), 2020).unwrap();
    a * b * c
}

fn main() {
    println!("Day 1:");
    println!("1: {}", part1());
    println!("2: {}", part2());
}
