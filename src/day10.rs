use std::ops::BitXor;
use util::hexidecimal;

struct KnotHash {
    array: Vec<u8>,
    position: usize,
    skipsize: usize,
}

impl KnotHash {
    fn new() -> KnotHash {
        KnotHash {
            array: (0..=255).collect(),
            position: 0,
            skipsize: 0,
        }
    }

    fn reverse(&mut self, min: usize, max: usize) {
        let len = self.array.len();
        let mid = (min + max) / 2;
        for (bot, top) in (min..mid).zip((mid..max).rev()) {
            self.array.swap(bot % len, top % len);
        }
    }

    fn round(&mut self, iter: impl IntoIterator<Item = usize>) {
        for len in iter {
            let pos = self.position;
            self.reverse(pos, pos + len);
            self.position += len + self.skipsize;
            self.skipsize += 1;
        }
    }

    fn hash(&self) -> String {
        hexidecimal(
            self.array
                .chunks(16)
                .map(|chunk| chunk.iter().fold(0, u8::bitxor)),
        )
    }
}

pub fn part1(input: &str) -> i32 {
    let mut knot = KnotHash::new();
    knot.round(input.split(',').map(|word| word.parse::<usize>().unwrap()));
    knot.array[0] as i32 * knot.array[1] as i32
}

static SUFFIX: &[usize] = &[17, 31, 73, 47, 23];

pub fn part2(input: &str) -> String {
    let lengths: Vec<usize> = input
        .bytes()
        .map(|b| b as usize)
        .chain(SUFFIX.iter().cloned())
        .collect();
    let mut knot = KnotHash::new();
    for _ in 0..64 {
        knot.round(lengths.iter().cloned());
    }
    knot.hash()
}

#[test]
fn examples() {
    let mut knot = KnotHash::new();
    knot.array = (0..5).collect();
    knot.round(vec![3, 4, 1, 5]);
    assert_eq!(vec![3, 4, 2, 1, 0], knot.array);

    assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", part2(""));
    assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", part2("AoC 2017"));
    assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", part2("1,2,3"));
    assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", part2("1,2,4"));
}
