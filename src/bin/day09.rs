use boolinator::Boolinator;

fn is_combination(n: u64, prev: &[u64]) -> bool {
    prev[..(prev.len() - 1)]
        .iter()
        .enumerate()
        .find_map(|(i, &p1)| prev[(i + 1)..].iter().find(|&&p2| n == p1 + p2))
        .is_some()
}

fn find_prefix(target: u64, range: &[u64]) -> Option<&[u64]> {
    range
        .iter()
        .scan(0, |total, &n| {
            *total += n;
            (*total <= target).as_some(*total)
        })
        .enumerate()
        .last()
        .and_then(|(i, n)| (n == target).as_some(&range[..(i + 1)]))
}

fn part1() -> (u64, Vec<u64>) {
    let numbers: Vec<_> = include_str!("../inputs/day09")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    let ans = numbers
        .iter()
        .enumerate()
        .skip(25)
        .find(|&(i, &n)| !is_combination(n, &numbers[(i - 25)..i]))
        .map(|(_, &n)| n)
        .unwrap();
    (ans, numbers)
}

fn part2(x1: u64, numbers: &[u64]) -> u64 {
    let (min, max) = (0..numbers.len())
        .find_map(|i| find_prefix(x1, &numbers[i..]))
        .unwrap()
        .iter()
        .fold((u64::MAX, u64::MIN), |(min, max), &n| {
            (min.min(n), max.max(n))
        });
    min + max
}

fn main() {
    println!("Day 8:");
    let (x1, numbers) = part1();
    println!("1: {}", x1);
    println!("2: {}", part2(x1, &numbers));
}
