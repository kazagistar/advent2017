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

fn main() {
    let input1 = get_file_string("data/input1.txt");
    println!("Day 1, part 1: {}", day1::part1(&input1));
    println!("Day 1, part 2: {}", day1::part2(&input1));

    let input2 = get_file_string("data/input2.txt");
    println!("Day 2, part 1: {}", day2::part1(&input2));
    println!("Day 2, part 2: {}", day2::part2(&input2));
}
