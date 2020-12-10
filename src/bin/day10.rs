fn parse_numbers() -> Vec<i32> {
    let mut numbers: Vec<_> = include_str!("../inputs/day10")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    numbers.sort_unstable();
    numbers.insert(0, 0);
    numbers.push(numbers.last().unwrap() + 3);
    numbers
}

fn part1(numbers: &[i32]) -> i32 {
    let (diff1, diff3) = numbers[1..]
        .iter()
        .scan(numbers[0], |prev, &n| {
            let diff = n - *prev;
            *prev = n;
            Some(diff)
        })
        .fold((0, 0), |(diff1, diff3), d| match d {
            1 => (diff1 + 1, diff3),
            3 => (diff1, diff3 + 1),
            _ => (diff1, diff3),
        });
    diff1 * diff3
}

fn part2(numbers: &[i32]) -> usize {
    let n = numbers.len();
    let mut counts = vec![0usize; n];
    counts[n - 1] = 1;
    for i in (0..(n - 1)).rev() {
        counts[i] = ((i + 1)..(n - 1))
            .take_while(|&k| numbers[k] <= numbers[i] + 3)
            .map(|k| counts[k as usize])
            .sum();
    }
    counts[0]
}

fn main() {
    println!("Day 10:");
    let numbers = parse_numbers();
    println!("1: {}", part1(&numbers));
    println!("2: {}", part2(&numbers));
}
