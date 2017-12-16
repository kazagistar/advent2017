use regex::Regex;
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

struct Dancers(Vec<char>);

impl Dancers {
    fn step(&mut self, step: Step) {
        match step {
            Spin(n) => {
                let mid = self.0.len() - n;
                self.0.rotate(mid);
            }
            Exchange(i, j) => self.0.swap(i, j),
            Partner(a, b) => {
                let i = self.0.iter().position(|&c| c == a).unwrap();
                let j = self.0.iter().position(|&c| c == b).unwrap();
                self.0.swap(i, j);
            }
        };
    }

    fn to_string(&self) -> String {
        self.0.iter().collect()
    }

    fn dance(&mut self, moves: impl IntoIterator<Item = Step>) {
        for m in moves {
            self.step(m);
        }
    }
}

impl<'a> From<&'a str> for Dancers {
    fn from(input: &str) -> Self {
        Dancers(input.chars().collect())
    }
}

const A2P: &'static str = "abcdefghijklmnop";

pub fn part1(input: &str) -> String {
    // Rust doesn't implement Step for char, which means we cant range over em... :(
    let mut dancers = Dancers::from(A2P);
    dancers.dance(parse_steps(input));
    dancers.to_string()
}

pub fn part2(input: &str) -> String {
    let moves: Vec<_> = parse_steps(input).collect();
    let mut dancers = Dancers::from(A2P);
    for i in 0..4_000_000_000usize { // wait a few months lul
        dancers.dance(moves.iter().cloned());
        if i % 100000 == 0 {
            println!("Iteration: {}", i);
        }
    }
    dancers.to_string()
}

#[test]
fn examples() {
    let moves: Vec<_> = parse_steps("s1,x3/4,pe/b").collect();
    assert_eq!(vec![Spin(1), Exchange(3, 4), Partner('e', 'b')], moves);

    let mut dancers = Dancers::from("abcde");
    for m in moves {
        dancers.step(m);
    }
    assert_eq!("baedc", &dancers.to_string());
}
