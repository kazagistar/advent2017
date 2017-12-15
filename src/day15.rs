use itertools::iterate;

type Int = i64;

const MOD: Int = 2147483647;
const FACTOR_A: Int = 16807;
const FACTOR_B: Int = 48271;

fn generator(factor: Int) -> impl Fn(&Int) -> Int {
    move |&value| value * factor % MOD
}

fn judge(a: impl Iterator<Item = Int>, b: impl Iterator<Item = Int>, count: usize) -> usize {
    a.zip(b)
        .take(count)
        .filter(|&(a, b)| (a as u16) == (b as u16))
        .count()
}

pub fn part1(a: i64, b: i64) -> usize {
    judge(
        iterate(a, generator(FACTOR_A)),
        iterate(b, generator(FACTOR_B)),
        40_000_000,
    )
}

pub fn part2(a: i64, b: i64) -> usize {
    judge(
        iterate(a, generator(FACTOR_A)).filter(|x| x % 4 == 0),
        iterate(b, generator(FACTOR_B)).filter(|x| x % 8 == 0),
        5_000_000,
    )
}

#[test]
fn examples() {
    assert_eq!(588, part1(65, 8921));
    assert_eq!(309, part2(65, 8921));
}
