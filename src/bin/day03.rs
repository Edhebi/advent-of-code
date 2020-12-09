fn check_slope(input: &str, dx: usize, dy: usize) -> usize {
    let w = input.lines().nth(0).unwrap().len();

    input
        .lines()
        .step_by(dy)
        .enumerate()
        .filter(|&(y, s)| s.chars().nth((y * dx) % w).unwrap() == '#')
        .count()
}

fn part1() -> usize {
    let input = include_str!("../inputs/day03");
    check_slope(input, 3, 1)
}

fn part2() -> usize {
    let input = include_str!("../inputs/day03");
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(dx, dy)| check_slope(input, dx, dy))
        .product()
}

fn main() {
    println!("Day 3:");
    println!("1: {}", part1());
    println!("2: {}", part2());
}
