use std::ops::Add;
use std::collections::HashMap;
use self::Direction::*;
use util::GenIter;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Vector(i32, i32);

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

static AROUND: &'static [Vector] = &[
    Vector(0, 1),
    Vector(1, 0),
    Vector(0, -1),
    Vector(-1, 0),
    Vector(1, 1),
    Vector(1, -1),
    Vector(-1, -1),
    Vector(-1, 1),
];

impl Direction {
    fn step(self) -> Vector {
        match self {
            North => Vector(0, 1),
            East => Vector(1, 0),
            South => Vector(0, -1),
            West => Vector(-1, 0),
        }
    }

    fn left(self) -> Self {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Vector) -> Self {
        let Vector(x1, y1) = self;
        let Vector(x2, y2) = rhs;
        Vector(x1 + x2, y1 + y2)
    }
}

fn spiral() -> impl Iterator<Item = Vector> {
    GenIter::from(|| {
        let mut pos = Vector(0, 0);
        let mut dir = North;
        // 1 north, 1 west, 2 south, 2 east, 3 north, 3 west, 4 south, 4 east, etc...
        for stepsize in 1.. {
            for _ in 0..2 {
                for _ in 0..stepsize {
                    yield pos;
                    pos = pos + dir.step();
                }
                dir = dir.left();
            }
        }
        unreachable!()
    })
}

pub fn part1(target: u32) -> i32 {
    let Vector(x, y) = spiral().skip(target as usize - 1).next().unwrap();
    x.abs() + y.abs()
}

pub fn part2(target: u32) -> u32 {
    let mut marks = HashMap::new();
    let mut spiral = spiral();
    marks.insert(spiral.next().unwrap(), 1);

    loop {
        let position = spiral.next().unwrap();
        let total = AROUND
            .iter()
            .flat_map(|&nearby| marks.get(&(position + nearby)))
            .sum();
        if total > target {
            return total;
        }
        marks.insert(position, total);
    }
}

#[test]
fn examples() {
    assert_eq!(0, part1(1));
    assert_eq!(3, part1(12));
    assert_eq!(2, part1(23));
    assert_eq!(31, part1(1024));

    assert_eq!(304, part2(147));
    assert_eq!(806, part2(800));
}
