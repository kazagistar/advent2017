use regex::*;

lazy_static! {
    // Finds the next { or }, while ignoring the contents of garbage strings inside < > and accouting for escape character !
    // See https://www.debuggex.com/ for help understanding this madness ;)
    static ref NEXT_GROUP: Regex = Regex::new(r"(?:,|<(?:!.|[^!>])*>)*(\{|\})?").unwrap();
}

pub fn part1(input: &str) -> i32 {
    let mut sum = 0;
    let mut value = 0;
    for capture in NEXT_GROUP.captures_iter(input) {
        match capture.get(1).map(|c| c.as_str()) {
            Some("{") => {
                value += 1;
                sum += value;
            }
            Some("}") => {
                value -= 1;
            }
            _ => (),
        }
    }
    return sum;
}

pub fn part2(input: &str) -> i32 {
    let mut count = 0;
    let mut iter = input.chars();
    loop {
        if None == iter.find(|&c| c == '<') {
            break count;
        }
        loop {
            match iter.next().unwrap() {
                '!' => {
                    iter.next();
                }
                '>' => break,
                _ => count += 1,
            }
        }
    }
}
