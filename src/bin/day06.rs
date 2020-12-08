struct State<M: Fn(u32, u32) -> u32> {
    answers: u32,
    count: usize,
    init: u32,
    merge: M,
}

impl<M: Fn(u32, u32) -> u32> State<M> {
    pub fn new(init: u32, merge: M) -> Self {
        Self {
            answers: init,
            count: 0,
            init,
            merge,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn step(&mut self, line: &str) {
        if line.is_empty() {
            self.count += self.answers.count_ones() as usize;
            self.answers = self.init;
        } else {
            let answers = line
                .chars()
                .map(|c| 1 << (c as u32 - 'a' as u32))
                .fold(0, |a, b| a | b);
            self.answers = (self.merge)(self.answers, answers);
        }
    }

    pub fn step_last(&mut self) {
        self.count += self.answers.count_ones() as usize;
    }
}

fn common(init: u32, merge: impl Fn(u32, u32) -> u32) -> usize {
    let input = include_str!("../inputs/day06");
    let mut state = State::new(init, merge);
    input.lines().for_each(|line| state.step(line));
    state.step_last();
    state.count()
}

fn part1() -> usize {
    common(0, |a, b| a | b)
}

fn part2() -> usize {
    common(0x03FFFFFF, |a, b| a & b)
}

fn main() {
    println!("Day 6:");
    println!("1: {}", part1());
    println!("2: {}", part2());
}
