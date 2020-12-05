fn to_bit(c: char) -> u16 {
    match c {
        'B' | 'R' => 1,
        'F' | 'L' => 0,
        _ => panic!("unknown character in input"),
    }
}

fn to_id(line: &str) -> u16 {
    line.chars().map(to_bit).fold(0, |id, b| (id << 1) | b)
}

fn xored_range(min: u16, max: u16) -> u16 {
    let from_zero = |x| [x, 1, x + 1, 0][(x % 4) as usize];
    from_zero(min - 1) ^ from_zero(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn common<A>(init: A, fold: impl Fn(A, u16) -> A) -> A {
        include_str!("../inputs/day05")
            .lines()
            .filter(|&s| !s.is_empty())
            .map(to_id)
            .fold(init, fold)
    }

    #[test]
    fn part1() {
        let seat = common(0, std::cmp::max);
        println!("seat = {}", seat);
    }

    #[test]
    fn part2() {
        let (min, max, xor) = common((1023, 0, 0), |(min, max, xor), id| {
            (min.min(id), max.max(id), xor ^ id)
        });
        let seat = xored_range(min, max) ^ xor;
        println!("seat = {}", seat);
    }
}
