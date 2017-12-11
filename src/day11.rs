use std::ops::Add;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Vector(i32, i32);

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Vector) -> Self {
        let Vector(x1, y1) = self;
        let Vector(x2, y2) = rhs;
        Vector(x1 + x2, y1 + y2)
    }
}

fn step(direction: &str) -> Vector {
    match direction {
        "n" => Vector(0, 1),
        "nw" => Vector(-1, 0),
        "sw" => Vector(-1, -1),
        "s" => Vector(0, -1),
        "se" => Vector(1, 0),
        "ne" => Vector(1, 1),
        _ => panic!("Bad input"),
    }
}

fn hex_distance(&Vector(x, y): &Vector) -> i32 {
    if x * y < 0 {
        x.abs() + y.abs()
    } else {
        x.abs().max(y.abs())
    }
}

pub fn part1(input: &str) -> i32 {
    hex_distance(&input
        .split(",")
        .map(step)
        .fold(Vector(0, 0), |pos, step| pos + step))
}

pub fn part2(input: &str) -> i32 {
    let mut pos = Vector(0, 0);
    let mut furthest = 0;
    for vec in input.split(",").map(step) {
        pos = pos + vec;
        furthest = furthest.max(hex_distance(&pos));
    }
    furthest
}
