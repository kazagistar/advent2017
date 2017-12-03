use std::fs::File;
use std::io::prelude::*;

pub fn get_file_string(path: &str) -> String {
    let mut result = String::new();
    let err = do catch { File::open(path)?.read_to_string(&mut result) };
    match err {
        Err(e) => panic!("Failed to read from file {} because {}", path, e),
        Ok(_) => result,
    }
}

// I found a compiler bug!
// pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = impl Iterator<Item = i32> + 'a> + 'a {
//     input.lines().map(|line| {
//         line.split_whitespace().map(|cell| cell.parse().unwrap())
//     })
// }

// Oldschool workaround...
use std::str::{Lines, SplitWhitespace};

pub fn spreadsheet_parse(input: &str) -> SheetParser {
    SheetParser { wrapped: input.lines() }
}

#[derive(Clone, Debug)]
pub struct SheetParser<'a> {
    wrapped: Lines<'a>,
}

impl <'a> Iterator for SheetParser<'a> {
    type Item = LineParser<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(LineParser { wrapped: self.wrapped.next()?.split_whitespace() })
    }
}

#[derive(Clone, Debug)]
pub struct LineParser<'a> {
    wrapped: SplitWhitespace<'a>,
}

impl <'a> Iterator for LineParser<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.wrapped.next().map(|cell| cell.parse().unwrap())
    }
}
