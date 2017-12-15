type Int = i64;

struct Generator {
    value: Int,
    factor: Int,
}

static MOD: Int = 2147483647;

impl Iterator for Generator {
    type Item = Int;
    fn next(&mut self) -> Option<Self::Item> {
        self.value = self.value * self.factor % MOD;
        Some(self.value)
    }
}

pub fn part1(a: i64, b: i64) -> usize {
    let gen_a = Generator {
        value: a,
        factor: 16807,
    };
    let gen_b = Generator {
        value: b,
        factor: 48271,
    };
    gen_a
        .zip(gen_b)
        .take(40_000_000)
        .filter(|&(a, b)| (a as u16) == (b as u16))
        .count()
}

pub fn part2(a: i64, b: i64) -> usize {
    let gen_a = Generator {
        value: a,
        factor: 16807,
    }.filter(|x| x % 4 == 0);
    let gen_b = Generator {
        value: b,
        factor: 48271,
    }.filter(|x| x % 8 == 0);
    gen_a
        .zip(gen_b)
        .take(5_000_000)
        .filter(|&(a, b)| (a as u16) == (b as u16))
        .count()
}
