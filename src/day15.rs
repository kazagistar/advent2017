use itertools::unfold;

type Int = i64;

const MOD: Int = 2147483647;
const FACTOR_A: Int = 16807;
const FACTOR_B: Int = 48271;

fn generator(factor: Int) -> impl Fn(&mut Int) -> Option<Int> {
    move |value| {
        *value = *value * factor % MOD;
        Some(*value)
    }
}

fn judge(a: impl Iterator<Item = Int>, b: impl Iterator<Item = Int>, count: usize) -> usize {
    a.zip(b)
        .take(count)
        .filter(|&(a, b)| (a as u16) == (b as u16))
        .count()
}

pub fn part1(a: i64, b: i64) -> usize {
    judge(
        unfold(a, generator(FACTOR_A)),
        unfold(b, generator(FACTOR_B)),
        40_000_000,
    )
}

pub fn part2(a: i64, b: i64) -> usize {
    judge(
        unfold(a, generator(FACTOR_A)).filter(|x| x % 4 == 0),
        unfold(b, generator(FACTOR_B)).filter(|x| x % 8 == 0),
        5_000_000,
    )
}
