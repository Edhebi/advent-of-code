const W: usize = 31;

fn check_slope(input: &str, dx: usize, dy: usize) -> usize {
    input
        .lines()
        .step_by(dy)
        .enumerate()
        .filter(|&(y, s)| {
            let x = (y * dx) % W;
            s.chars().nth(x).unwrap() == '#'
        })
        .count()
}

fn main() {
    let input = include_str!("INPUT");
    let n: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(dx, dy)| check_slope(input, dx, dy))
        .product();

    print!("n = {}", n);
}
