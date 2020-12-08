use regex::Regex;
#[derive(Copy, Clone)]
struct State<V: Fn(&str, &str) -> bool> {
    fields: Option<u8>,
    count: usize,
    validator: V,
}

impl<V: Fn(&str, &str) -> bool> State<V> {
    pub fn new(validator: V) -> Self {
        Self {
            fields: Some(0),
            count: 0,
            validator,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    fn valid_fields(&self) -> bool {
        self.fields.map(|f| f & 0x7F == 0x7F).unwrap_or(false)
    }

    pub fn step(&mut self, line: &str) {
        if line.is_empty() {
            if self.valid_fields() {
                self.count += 1;
            }
            self.fields = Some(0);
        } else {
            self.fields = self.fields.and_then(|f| self.parse(line).map(|g| f | g));
        }
    }

    pub fn step_last(&mut self) {
        if self.valid_fields() {
            self.count += 1;
        }
    }

    fn validate_segment(&self, key: &str, value: &str) -> Option<u8> {
        let fields = match key {
            "byr" => 1 << 0,
            "iyr" => 1 << 1,
            "eyr" => 1 << 2,
            "hgt" => 1 << 3,
            "hcl" => 1 << 4,
            "ecl" => 1 << 5,
            "pid" => 1 << 6,
            "cid" => 1 << 7,
            _ => return None,
        };
        if (self.validator)(key, value) {
            Some(fields)
        } else {
            None
        }
    }

    fn parse(&self, line: &str) -> Option<u8> {
        let mut fields = 0u8;
        for segment in line.split_ascii_whitespace() {
            let key = segment.split(':').nth(0)?;
            let value = segment.split(':').nth(1)?;
            fields |= self.validate_segment(key, value)?;
        }
        Some(fields)
    }
}

fn common(validator: impl Fn(&str, &str) -> bool) -> usize {
    let input = include_str!("../inputs/day04");
    let mut state = State::new(validator);
    for line in input.lines() {
        state.step(line);
    }
    state.step_last();
    state.count()
}

fn part1() -> usize {
    common(|_, _| true)
}

fn part2() -> usize {
    let hgt_re = Regex::new("^([0-9]+)(cm|in)$").unwrap();
    let hcl_re = Regex::new("^#[0-9a-f]{6}$").unwrap();
    let pid_re = Regex::new("^[0-9]{9}$").unwrap();

    common(|key, value| match key {
        "byr" => (1920..=2002).contains(&value.parse().unwrap_or(0)),
        "iyr" => (2010..=2020).contains(&value.parse().unwrap_or(0)),
        "eyr" => (2020..=2030).contains(&value.parse().unwrap_or(0)),
        "hgt" => hgt_re
            .captures(value)
            .map(|c| {
                let h = c.get(1).unwrap().as_str().parse().unwrap();
                match c.get(2).unwrap().as_str() {
                    "cm" => (150..=193).contains(&h),
                    "in" => (59..=76).contains(&h),
                    _ => false,
                }
            })
            .unwrap_or(false),
        "hcl" => hcl_re.is_match(value),
        "ecl" => match value {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        },
        "pid" => pid_re.is_match(value),
        "cid" => true,
        _ => false,
    })
}

fn main() {
    println!("Day 4:");
    println!("1: {}", part1());
    println!("2: {}", part2());
}
