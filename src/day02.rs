#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    character: char,
}

fn parse(line: &str) -> Option<(Policy, &str)> {
    let segments: Vec<&str> = line.split(' ').collect();
    let minmax: Vec<&str> = segments.get(0)?.split('-').collect();
    let policy = Policy {
        min: minmax.get(0)?.parse().ok()?,
        max: minmax.get(1)?.parse().ok()?,
        character: segments.get(1)?.chars().nth(0)?,
    };
    Some((policy, *segments.get(2)?))
}

fn match_policy_1(policy: &Policy, pass: &str) -> bool {
    let n = pass.chars().filter(|&c| c == policy.character).count();
    n >= policy.min && n <= policy.max
}

fn match_policy_2(policy: &Policy, pass: &str) -> bool {
    [policy.min, policy.max]
        .iter()
        .filter_map(|&i| pass.chars().nth(i - 1))
        .filter(|&c| c == policy.character)
        .count()
        == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn common(matcher: impl Fn(&Policy, &str) -> bool) {
        let input = include_str!("../inputs/day02");
        let n = input
            .lines()
            .filter_map(parse)
            .filter(|pair| matcher(&pair.0, pair.1))
            .count();

        println!("n = {}", n);
    }

    #[test]
    fn part1() {
        common(match_policy_1)
    }

    #[test]
    fn part2() {
        common(match_policy_2)
    }
}
