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

fn main() {
    let input = include_str!("INPUT");
    let entries = input.lines().filter_map(|s| s.parse().ok()).collect();

    let (a, b) = sum2(&entries, 2020).unwrap();
    println!("SUM 2:");
    println!("a = {}, b = {}, a * b = {}", a, b, a * b);

    let (a, b, c) = sum3(&entries, 2020).unwrap();
    println!("SUM 3:");
    println!("a = {}, b = {}, c = {}, a * b * c = {}", a, b, c, a * b * c);
}
