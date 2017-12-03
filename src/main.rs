#![feature(catch_expr)]
#![feature(universal_impl_trait)]
#![feature(conservative_impl_trait)]
#![feature(generators)]
#![feature(generator_trait)]
#![feature(never_type)]

use std::fs::File;
use std::io::prelude::*;

mod day1;
mod day2;
mod day3;

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

fn get_file_string(path: &str) -> String {
    let mut result = String::new();
    let err = do catch { File::open(path)?.read_to_string(&mut result) };
    match err {
        Err(e) => panic!("Failed to read from file {} because {}", path, e),
        Ok(_) => result,
    }
}
