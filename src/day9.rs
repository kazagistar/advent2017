use regex::*;

lazy_static! {
    // Finds the next { or }, while ignoring the contents of garbage strings inside < > and accouting for escape character !
    // See https://www.debuggex.com/ for help understanding this madness ;)
    static ref NEXT_GROUP: Regex = Regex::new(r"(?:,|<(?:!.|[^!>])*>)*([{}])?").unwrap();
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
        if iter.find(|&c| c == '<') == None {
            return count;
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

#[test]
fn examples() {
    assert_eq!(1, part1("{}"));
    assert_eq!(6, part1("{{{}}}"));
    assert_eq!(5, part1("{{},{}}"));
    assert_eq!(16, part1("{{{},{},{{}}}}"));
    assert_eq!(1, part1("{<a>,<a>,<a>,<a>}"));
    assert_eq!(9, part1("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
    assert_eq!(9, part1("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
    assert_eq!(3, part1("{{<a!>},{<a!>},{<a!>},{<ab>}}"));

    assert_eq!(0, part2("{<>}"));
    assert_eq!(17, part2("{<random characters>}"));
    assert_eq!(3, part2("{<<<<>}"));
    assert_eq!(2, part2("{<{!>}>}"));
    assert_eq!(0, part2("{<!!>}"));
    assert_eq!(0, part2("{<!!!>>}"));
    assert_eq!(10, part2("{<{o\"i!a,<{i<a>}"));
}
