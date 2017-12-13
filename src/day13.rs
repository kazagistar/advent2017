fn parse(line: &str) -> (usize, usize) {
    let mut parts = line.trim().split(": ").map(|word| word.parse().unwrap());
    (parts.next().unwrap(), parts.next().unwrap())
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (depth, range) = parse(line);
            let cycle = (range - 1) * 2;
            if depth % cycle == 0 {
                depth * range
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let bads: Vec<_> = input
        .lines()
        .map(|line| {
            let (depth, range) = parse(line);
            let cycle = (range - 1) * 2;
            (depth, cycle)
        })
        .collect();
    (0..)
        .find(|time| {
            !bads.iter()
                .any(move |&(depth, cycle)| (depth + time) % cycle == 0)
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
