fn check_slope(input: &str, dx: usize, dy: usize) -> usize {
    const W: usize = 31;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = include_str!("../inputs/day03");
        print!("n = {}", check_slope(input, 3, 1));
    }

    #[test]
    fn part2() {
        let input = include_str!("../inputs/day03");
        let n: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|&(dx, dy)| check_slope(input, dx, dy))
            .product();

        print!("n = {}", n);
    }
}
