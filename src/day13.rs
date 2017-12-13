fn parse(line: &str) -> (usize, usize) {
    let mut parts = line.trim().split(": ").map(|word| word.parse().unwrap());
    (parts.next().unwrap(), parts.next().unwrap())
}

fn pos(range: usize, time: usize) -> usize {
    let full = (range - 1) * 2;
    let cycle = time % full;
    usize::min(cycle, full - cycle)
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (depth, range) = parse(line);
            if pos(range, depth) == 0 {
                depth * range
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &str) -> &str {
    "?"
}

#[test]
fn examples() {
    let input = "0: 3
                 1: 2
                 4: 4
                 6: 4";
    assert_eq!(24, part1(input));
}
