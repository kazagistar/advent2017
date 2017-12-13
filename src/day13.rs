struct Firewall {
    depth: usize,
    range: usize,
}

impl Firewall {
    fn cycle(&self) -> usize {
        (self.range - 1) * 2
    }
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item = Firewall> + 'a {
    input.lines().map(|line| {
        let mut parts = line.trim().split(": ").map(|word| word.parse().unwrap());
        Firewall {
            depth: parts.next().unwrap(),
            range: parts.next().unwrap(),
        }
    })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .map(|layer| {
            if layer.depth % layer.cycle() == 0 {
                layer.depth * layer.range
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let layers: Vec<Firewall> = parse(input).collect();
    (0..)
        .find(|time| {
            !layers
                .iter()
                .any(move |layer| (layer.depth + time) % layer.cycle() == 0)
        })
        .unwrap()
}

#[test]
fn examples() {
    let input = "0: 3
                 1: 2
                 4: 4
                 6: 4";
    assert_eq!(24, part1(input));
    assert_eq!(10, part2(input));
}
