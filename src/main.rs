#![feature(catch_expr)]
#![feature(universal_impl_trait)]

use std::fs::File;
use std::io::prelude::*;

fn get_file_string(path: &str) -> String {
    let mut result = String::new();
    let err = do catch { File::open(path)?.read_to_string(&mut result) };
    match err {
        Err(e) => panic!("Failed to read from file {} because {}", path, e),
        Ok(_) => result,
    }
}

mod day1 {
    // Zip togeather the items with a given offset, and if they are equal, sum them
    pub fn sum_matching(input: &str, offset: usize) -> u32 {
        input
            .chars()
            .zip(input.chars().cycle().skip(offset))
            .filter(|&(a, b)| a == b)
            .map(|x| x.1.to_digit(10).unwrap())
            .sum()
    }

    pub fn part1(input: &str) -> u32 {
        sum_matching(input, 1)
    }

    pub fn part2(input: &str) -> u32 {
        sum_matching(input, input.len() / 2)
    }

    #[test]
    fn examples() {
        assert_eq!(3, part1("1122"));
        assert_eq!(4, part1("1111"));
        assert_eq!(0, part1("1234"));
        assert_eq!(9, part1("91212129"));

        assert_eq!(6, part2("1212"));
        assert_eq!(0, part2("1221"));
        assert_eq!(4, part2("123425"));
        assert_eq!(12, part2("123123"));
        assert_eq!(4, part2("12131415"));
    }
}

mod day2 {
    fn parser(input: &str) -> Vec<Vec<i32>> {
        input
            .split('\n')
            .map(|line| {
                line.split('\t')
                    .flat_map(|cell| cell.parse())
                    .collect::<Vec<_>>()
            })
            .filter(|row| row.len() != 0)
            .collect()
    }

    pub fn part1(input: &str) -> i32 {
        parser(input)
            .iter()
            .map(|v| v.iter().max().unwrap() - v.iter().min().unwrap())
            .sum()
    }

    fn combinations(input: &[i32]) -> Vec<(i32, i32)> {
        let mut result = Vec::new();
        let mut iter = input.iter().cloned();
        while let Some(x) = iter.next() {
            for y in iter.clone() {
                result.push((x, y));
            }
        }
        result
    }

    pub fn part2(input: &str) -> i32 {
        parser(input)
            .iter()
            .map(|v| {
                combinations(&v)
                    .iter()
                    .map(|&(x, y)| {
                        if x % y == 0 {
                            x / y
                        } else if y % x == 0 {
                            y / x
                        } else {
                            0
                        }
                    })
                    .sum::<i32>()
            })
            .sum()
    }
}

mod day3 {
    use std::ops::Add;
    use std::collections::HashMap;

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
                Direction::North => Vector(0, 1),
                Direction::East => Vector(1, 0),
                Direction::South => Vector(0, -1),
                Direction::West => Vector(-1, 0),
            }
        }

        fn left(self) -> Self {
            match self {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
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

    fn spiral_marking(target: u32) -> u32 {
        let mut pos = Vector(0, 0);
        let mut dir = Direction::North;
        let mut marks = HashMap::new();
        marks.insert(pos, 1);

        for stepsize in 1.. {
            for _ in 0..2 {
                for _ in 0..stepsize {
                    pos = pos + dir.step();
                    let total: u32 = AROUND
                        .iter()
                        .flat_map(|&nearby| marks.get(&(pos + nearby)).map(|x| *x))
                        .sum();
                    if total > target {
                        return total;
                    }
                    marks.insert(pos, total);
                }
                dir = dir.left();
            }
        }
        unreachable!()
    }

    fn spiral(target: u32) -> Vector {
        let mut pos = Vector(0, 0);
        let mut dir = Direction::North;
        let mut idx = 1;

        for stepsize in 1.. {
            for _ in 0..2 {
                for _ in 0..stepsize {
                    if idx == target {
                        return pos;
                    }
                    pos = pos + dir.step();
                    idx += 1;
                }
                dir = dir.left();
            }
        }
        unreachable!()
    }

    pub fn part1(target: u32) -> i32 {
        let Vector(x, y) = spiral(target);
        x.abs() + y.abs()
    }

    pub fn part2(target: u32) -> u32 {
        spiral_marking(target)
    }
}

fn main() {
    let input1 = get_file_string("data/input1.txt");
    println!("Day 1, part 1: {}", day1::part1(&input1));
    println!("Day 1, part 2: {}", day1::part2(&input1));

    let input2 = get_file_string("data/input2.txt");
    println!("Day 2, part 1: {}", day2::part1(&input2));
    println!("Day 2, part 2: {}", day2::part2(&input2));

    let input3 = 277678;
    println!("Day 3, part 1: {}", day3::part1(input3));
    println!("Day 3, part 2: {}", day3::part2(input3));
}
