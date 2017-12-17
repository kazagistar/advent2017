use std::collections::HashMap;
use std::ops::Add;
use regex::Regex;
use itertools::{iterate, unfold};
use self::Step::*;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Step {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

lazy_static! {
    static ref MATCHER: Regex = Regex::new(r"x(\d+)/(\d+)|p([a-p])/([a-p])|s(\d+)").unwrap();
}

fn parse_steps<'a>(steps: &'a str) -> impl Iterator<Item = Step> + 'a {
    MATCHER.captures_iter(steps).map(|cap| {
        let captures: Vec<_> = cap.iter().map(|o| o.map(|m| m.as_str())).collect();
        match &captures[1..] {
            &[Some(a), Some(b), _, _, _] => Exchange(a.parse().unwrap(), b.parse().unwrap()),
            &[_, _, Some(i), Some(j), _] => {
                Partner(i.chars().next().unwrap(), j.chars().next().unwrap())
            }
            &[_, _, _, _, Some(c)] => Spin(c.parse().unwrap()),
            _ => unreachable!(),
        }
    })
}

#[derive(Debug)]
struct MultiStep {
    // both of these represent a backwards (target => source) mapping
    shuffle: Vec<usize>,
    rename: HashMap<char, char>,
}

impl MultiStep {
    fn new(size: usize) -> MultiStep {
        MultiStep {
            shuffle: (0..size).collect(),
            rename: HashMap::with_capacity(size),
        }
    }

    fn step(&mut self, step: Step) {
        match step {
            Spin(n) => {
                let mid = self.shuffle.len() - n;
                self.shuffle.rotate(mid);
            }
            Exchange(i, j) => self.shuffle.swap(i, j),
            Partner(ref a, ref b) => {
                let ta = *self.rename.get(a).unwrap_or(a);
                let tb = *self.rename.get(b).unwrap_or(b);
                self.rename.insert(*a, tb);
                self.rename.insert(*b, ta);
            }
        }
    }

    fn steps(&mut self, steps: impl IntoIterator<Item = Step>) {
        for step in steps {
            self.step(step);
        }
    }

    fn apply_shuffle<'a, T: Clone>(&'a self, dancers: &'a [T]) -> impl Iterator<Item = T> + 'a {
        self.shuffle.iter().map(move |&i| dancers[i].clone())
    }

    fn dance(&self, dancers: &[char]) -> String {
        self.apply_shuffle(dancers)
            .map(|c| {
                self.rename
                    .iter()
                    .find(|&(_, &v)| c == v)
                    .map(|(&k, _)| k)
                    .unwrap_or(c)
            })
            .collect()
    }
}

impl<'a> Add for &'a MultiStep {
    type Output = MultiStep;
    fn add(self, rhs: Self) -> MultiStep {
        assert_eq!(self.shuffle.len(), rhs.shuffle.len());
        MultiStep {
            shuffle: self.apply_shuffle(&rhs.shuffle).collect(),
            rename: rhs.rename
                .iter()
                .map(|(&target, source)| {
                    (target, *self.rename.get(source).unwrap_or(source))
                })
                .collect(),
        }
    }
}

lazy_static! {
    static ref A2P: Vec<char> = "abcdefghijklmnop".chars().collect();
}

fn parse_dance(input: &str, size: usize) -> MultiStep {
    let mut dance = MultiStep::new(size);
    dance.steps(parse_steps(input));
    dance
}

fn iter_bits(st: u32) -> impl Iterator<Item = bool> {
    unfold(st, |st| {
        let out = *st;
        *st = out >> 1;
        if out != 0 {
            Some(out & 1 != 0)
        } else {
            None
        }
    })
}

fn many_dance(times: u32, single: MultiStep) -> MultiStep {
    let doubling_dances = iterate(single, |dance| dance + dance);
    let mut partial_dances = iter_bits(times)
        .zip(doubling_dances)
        .filter(|&(bit, _)| bit)
        .map(|(_, dance)| dance);
    let mut full = partial_dances.next().unwrap();
    partial_dances.for_each(|partial| full = &full + &partial);
    full
}

pub fn part1(input: &str) -> String {
    parse_dance(input, A2P.len()).dance(&A2P)
}

pub fn part2(input: &str) -> String {
    many_dance(4_000_000_000, parse_dance(input, A2P.len())).dance(&A2P)
}

#[test]
fn examples() {
    let input = "s1,x3/4,pe/b";
    let moves: Vec<_> = parse_steps(input).collect();
    assert_eq!(vec![Spin(1), Exchange(3, 4), Partner('e', 'b')], moves);

    let start: Vec<_> = "abcde".chars().collect();
    assert_eq!("baedc", &parse_dance(input, start.len()).dance(&start));

    assert_eq!(
        "ceadb",
        &many_dance(2, parse_dance(input, start.len())).dance(&start)
    );
}
