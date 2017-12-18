#![feature(catch_expr)]
#![feature(universal_impl_trait)]
#![feature(conservative_impl_trait)]
#![feature(generators)]
#![feature(generator_trait)]
#![feature(never_type)]
#![feature(range_contains)]
#![feature(slice_patterns)]
#![feature(inclusive_range_syntax)]
#![feature(slice_rotate)]

extern crate bit_vec;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate petgraph;
extern crate regex;

mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;

fn main() {
    let input1 = util::get_file_string("data/input1.txt");
    println!("Day 1, part 1: {}", day1::part1(&input1));
    println!("Day 1, part 2: {}", day1::part2(&input1));

    let input2 = util::get_file_string("data/input2.txt");
    println!("Day 2, part 1: {}", day2::part1(&input2));
    println!("Day 2, part 2: {}", day2::part2(&input2));

    let input3 = 277678;
    println!("Day 3, part 1: {}", day3::part1(input3));
    println!("Day 3, part 2: {}", day3::part2(input3));

    let input4 = util::get_file_string("data/input4.txt");
    println!("Day 4, part 1: {}", day4::part1(&input4));
    println!("Day 4, part 2: {}", day4::part2(&input4));

    let input5 = util::get_file_string("data/input5.txt");
    println!("Day 5, part 1: {}", day5::part1(&input5));
    println!("Day 5, part 2: {}", day5::part2(&input5));

    let input6 = util::get_file_string("data/input6.txt");
    println!("Day 6, part (1, 2): {:?}", day6::solve(&input6));

    let input7 = util::get_file_string("data/input7.txt");
    println!("Day 7, part 1: {:?}", day7::part1(&input7));
    println!("Day 7, part 2: {:?}", day7::part2(&input7));

    let input8 = util::get_file_string("data/input8.txt");
    println!("Day 8, part (1, 2): {:?}", day8::solve(&input8));

    let input9 = util::get_file_string("data/input9.txt");
    println!("Day 9, part 1: {:?}", day9::part1(&input9));
    println!("Day 9, part 2: {:?}", day9::part2(&input9));

    let input10 = util::get_file_string("data/input10.txt");
    println!("Day 10, part 1: {:?}", day10::part1(&input10));
    println!("Day 10, part 2: {:?}", day10::part2(&input10));

    let input11 = util::get_file_string("data/input11.txt");
    println!("Day 11, part 1: {:?}", day11::part1(&input11));
    println!("Day 11, part 2: {:?}", day11::part2(&input11));

    let input12 = util::get_file_string("data/input12.txt");
    println!("Day 12, part 1: {:?}", day12::part1(&input12));
    println!("Day 12, part 2: {:?}", day12::part2(&input12));

    let input13 = util::get_file_string("data/input13.txt");
    println!("Day 13, part 1: {:?}", day13::part1(&input13));
    println!("Day 13, part 2: {:?}", day13::part2(&input13));

    let input14 = "xlqgujun";
    println!("Day 14, part 1: {:?}", day14::part1(&input14));
    println!("Day 14, part 2: {:?}", day14::part2(&input14));

    let input15 = (873, 583);
    println!("Day 15, part 1: {:?}", day15::part1(input15.0, input15.1));
    println!("Day 15, part 2: {:?}", day15::part2(input15.0, input15.1));

    let input16 = util::get_file_string("data/input16.txt");
    println!("Day 16, part 1: {:?}", day16::part1(&input16));
    println!("Day 16, part 2: {:?}", day16::part2(&input16));

    let input17 = 328;
    println!("Day 17, part 1: {:?}", day17::part1(input17));
    println!("Day 17, part 2: {:?}", day17::part2(input17));

    let input18 = util::get_file_string("data/input18.txt");
    println!("Day 18, part 1: {:?}", day18::part1(&input18));
    println!("Day 18, part 2: {:?}", day18::part2(&input18));
}
