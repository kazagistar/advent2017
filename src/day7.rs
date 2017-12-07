use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref MATCHER: Regex = Regex::new(r"^(\w+) \((\d+)\)(?: -> (.*))?$").unwrap();
}

pub fn part1(input: &str) -> &str {
    let mut all_names = HashSet::new();
    let mut all_children = HashSet::new();
    for (name, _, children) in input.lines().map(parse_node) {
        all_names.insert(name);
        all_children.extend(children);
    }
    all_names.difference(&all_children).next().unwrap()
}

fn parse_node(input: &str) -> (&str, i32, Vec<&str>) {
    let captures = MATCHER.captures(input).unwrap();
    let name = captures.get(1).unwrap().as_str();
    let weight = captures.get(2).unwrap().as_str().parse().unwrap();
    let children = captures
        .get(3)
        .map(|x| x.as_str().split(", ").collect())
        .unwrap_or_else(Vec::new);
    (name, weight, children)
}
