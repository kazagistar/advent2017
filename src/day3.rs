use std::ops::Add;
use std::ops::{Generator, GeneratorState};
use std::collections::HashMap;
use self::Direction::*;

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
    let generator = move || {
        let mut pos = Vector(0, 0);
        let mut dir = Direction::North;
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
    };
    GenIter { generator }
}

struct GenIter<T: Generator> {
    generator: T,
}

impl<T: Generator> Iterator for GenIter<T> {
    type Item = T::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match self.generator.resume() {
            GeneratorState::Yielded(value) => Some(value),
            GeneratorState::Complete(_) => None,
        }
    }
}

pub fn part1(target: u32) -> i32 {
    let Vector(x, y) = spiral().skip(target as usize - 1).next().unwrap();
    x.abs() + y.abs()
}

pub fn part2(target: u32) -> u32 {
    let mut marks = HashMap::new();
    marks.insert(Vector(0, 0), 1);

    for position in spiral().skip(1) {
        let total: u32 = AROUND
            .iter()
            .flat_map(|&nearby| marks.get(&(position + nearby)).map(|x| *x))
            .sum();
        if total > target {
            return total;
        }
        marks.insert(position, total);
    }
    unreachable!()
}
