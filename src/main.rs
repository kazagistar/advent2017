#![feature(catch_expr)]
#![feature(universal_impl_trait)]
#![feature(conservative_impl_trait)]
#![feature(generators)]
#![feature(generator_trait)]
#![feature(never_type)]
#![feature(range_contains)]
#![feature(slice_patterns)]

#[macro_use]
extern crate lazy_static;
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
}
