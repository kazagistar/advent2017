use std::fs::File;
use std::io::prelude::*;
use std::fmt::{Write, LowerHex, Binary};
use std::mem::size_of;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign, Generator, GeneratorState};
use std::iter::Sum;

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

pub fn hexidecimal<T>(array: impl IntoIterator<Item = T>) -> String where T: LowerHex {
    let per_item = size_of::<T>() * 2;
    let mut buff = String::new();
    for item in array {
        write!(&mut buff, "{:0width$x}", item, width = per_item).unwrap();
    }
    buff
}

pub fn binary<T>(array: impl IntoIterator<Item = T>) -> String where T: Binary {
    let per_item = size_of::<T>() * 8;
    let mut buff = String::new();
    for item in array {
        write!(&mut buff, "{:0width$b}", item, width = per_item).unwrap();
    }
    buff
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Default)]
pub struct Vector(pub i32, pub i32);

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Vector) -> Self {
        let Vector(x1, y1) = self;
        let Vector(x2, y2) = rhs;
        Vector(x1 + x2, y1 + y2)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        *self = *self + other;
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        let Vector(x1, y1) = self;
        Vector(-x1, -y1)
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Vector) -> Self {
        self + (-rhs)
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Vector) {
        *self = *self - other;
    }
}

impl Sum for Vector {
    fn sum<I>(iter: I) -> Self where I: Iterator<Item = Vector> {
        iter.fold(Vector::default(), Add::add)
    }
}
