struct Loop {
    array: Vec<i32>,
}

impl Loop {
    fn new(len: i32) -> Loop {
        Loop {
            array: (0..len).collect(),
        }
    }

    fn reverse(&mut self, min: usize, max: usize) {
        let len = self.array.len();
        let mid = (min + max) / 2;
        for (i, j) in (min..mid)
            .map(|i| i % len)
            .zip((mid..max).rev().map(|i| i % len))
        {
            self.array.swap(i, j);
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let mut knot = Loop::new(256);
    let mut init = 0;
    let mut skipsize = 0;
    for len in input.split(',').map(|word| word.parse::<usize>().unwrap()) {
        knot.reverse(init, init + len);
        init += len + skipsize;
        skipsize += 1;
    }
    knot.array[0] * knot.array[1]
}
