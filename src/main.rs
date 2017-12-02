#![feature(catch_expr)]
#![feature(universal_impl_trait)]

#[macro_use]
extern crate rulinalg;

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
    use rulinalg::matrix::{BaseMatrix, Matrix};

    fn parser(input: &str) -> Matrix<i32> {
        let lists = input
            .split('\n')
            .map(|line| {
                line.split('\t').map(|cell| cell.parse().unwrap()).collect()
            })
            .collect();
        let height = lists.len();
        let width = lists[0].len();
        Matrix::new(height, width, lists.iter().flat_map(|x| x))
    }

    fn min_max_diff(list: impl Iterator<Item = &i32> + Clone) -> Option<i32> {
        Some(list.max()? - list.min()?)
    }

    fn part1(input: &str) -> i32 {
        let array = parser(input);
        let horizontals = array.row_iter().map(|x| min_max_diff(x.iter())).sum();
        let verticals = array.col_iter().map(|x| min_max_diff(x.iter())).sum();
        horizontals + verticals
    }
}

fn main() {
    let input1 = get_file_string("data/input1.txt");
    println!("Day 1, part 1: {}", day1::part1(&input1));
    println!("Day 1, part 2: {}", day1::part2(&input1));

    let input2 = get_file_string("data/input2.txt");
}
