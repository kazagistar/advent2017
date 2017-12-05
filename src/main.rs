#![feature(catch_expr)]
#![feature(universal_impl_trait)]
#![feature(conservative_impl_trait)]
#![feature(generators)]
#![feature(generator_trait)]
#![feature(never_type)]

mod input;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let input1 = input::get_file_string("data/input1.txt");
    println!("Day 1, part 1: {}", day1::part1(&input1));
    println!("Day 1, part 2: {}", day1::part2(&input1));

    let input2 = input::get_file_string("data/input2.txt");
    println!("Day 2, part 1: {}", day2::part1(&input2));
    println!("Day 2, part 2: {}", day2::part2(&input2));

    let input3 = 277678;
    println!("Day 3, part 1: {}", day3::part1(input3));
    println!("Day 3, part 2: {}", day3::part2(input3));

    let input4 = input::get_file_string("data/input4.txt");
    println!("Day 4, part 1: {}", day4::part1(&input4));
    println!("Day 4, part 2: {}", day4::part2(&input4));

    let input5 = input::get_file_string("data/input5.txt");
    println!("Day 5, part 1: {}", day5::part1(&input5));
    println!("Day 5, part 2: {}", day5::part2(&input5));
}
