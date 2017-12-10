use std::fs::File;
use std::io::prelude::*;
use std::ops::{Generator, GeneratorState};
use std::fmt::LowerHex;
use std::fmt::Write;
use std::mem::size_of;

pub fn get_file_string(path: &str) -> String {
    let mut result = String::new();
    let err = do catch { File::open(path)?.read_to_string(&mut result) };
    match err {
        Err(e) => panic!("Failed to read from file {} because {}", path, e),
        Ok(_) => result,
    }
}

pub struct GenIter<T: Generator> {
    generator: T,
}

impl <T: Generator> From<T> for GenIter<T> {
    fn from(generator: T) -> GenIter<T> {
        GenIter { generator }
    }
}

// I feel like this should be in the standard library (as generator.into_iter()), but I guess generator trait is still experimental
impl<T: Generator> Iterator for GenIter<T> {
    type Item = T::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match self.generator.resume() {
            GeneratorState::Yielded(value) => Some(value),
            GeneratorState::Complete(_) => None,
        }
    }
}

pub fn hex<T>(array: impl IntoIterator<Item = T>) -> String where T: LowerHex {
    let per_item = size_of::<T>() * 2;
    let mut buff = String::new();
    for item in array {
        write!(&mut buff, "{:0width$x}", item, width = per_item).unwrap();
    }
    buff
}
