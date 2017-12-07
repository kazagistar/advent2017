use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> &str {
    find_root(&parse_programs(input))
}

pub fn part2(input: &str) -> usize {
    let programs = parse_programs(input);
    find_bad(&programs, find_root(&programs)).unwrap_err()
}

lazy_static! {
    static ref MATCHER: Regex = Regex::new(r"^(\w+) \((\d+)\)(?: -> (.*))?$").unwrap();
}

type Weight = usize;

struct Program<'a> {
    weight: Weight,
    children: Vec<&'a str>,
}

fn parse_line(input: &str) -> (&str, Program) {
    let captures = MATCHER.captures(input).unwrap();
    let name = captures.get(1).unwrap().as_str();
    let weight = captures.get(2).unwrap().as_str().parse().unwrap();
    let children = captures
        .get(3)
        .map(|x| x.as_str().split(", ").collect())
        .unwrap_or_else(Vec::new);
    (name, Program { weight, children })
}

fn parse_programs(input: &str) -> HashMap<&str, Program> {
    input.lines().map(parse_line).collect()
}

fn find_root<'a, 'b>(programs: &'b HashMap<&'a str, Program<'a>>) -> &'a str {
    let mut all_names: HashSet<&'a str> = HashSet::new();
    let mut all_children: HashSet<&'a str> = HashSet::new();
    for (name, program) in programs {
        all_names.insert(name);
        all_children.extend(program.children.iter().cloned());
    }
    all_names.difference(&all_children).next().unwrap()
}

// Ok = child weight, Err = fixed bad weight
fn find_bad(programs: &HashMap<&str, Program>, name: &str) -> Result<Weight, Weight> {
    let program = programs.get(name).unwrap();
    if program.children.is_empty() {
        return Ok(program.weight);
    }

    let mut bunched = HashMap::new();
    for child in program.children.iter().cloned() {
        let weight = find_bad(programs, child)?;
        bunched.entry(weight).or_insert_with(Vec::new).push(child);
    }
    if bunched.len() == 1 {
        let children = program.children.len();
        let weight_per_child = bunched.keys().next().unwrap();
        return Ok(children * weight_per_child + program.weight);
    }

    let (bad_weight, bad) = bunched.iter().min_by_key(|&(_, c)| c.len()).unwrap();
    let (good_weight, _) = bunched.iter().max_by_key(|&(_, c)| c.len()).unwrap();
    let difference = good_weight - bad_weight;
    let fixed = programs.get(bad[0]).unwrap().weight + difference;
    Err(fixed)
}
