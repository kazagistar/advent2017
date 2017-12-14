use day10::KnotHash;
use std::collections::{HashMap, HashSet};
use util::Vector;
use petgraph::unionfind::UnionFind;

pub fn part1(input: &str) -> usize {
    get_filled(input).count()
}

static AROUND: &'static [Vector] = &[Vector(0, 1), Vector(1, 0), Vector(0, -1), Vector(-1, 0)];

pub fn part2(input: &str) -> usize {
    let disk: HashMap<Vector, usize> = get_filled(input).zip(0..).collect();
    let mut groups = UnionFind::new(disk.len());
    for (&filled, &index) in disk.iter() {
        for &direction in AROUND {
            if let Some(&neighbor) = disk.get(&(filled + direction)) {
                groups.union(index, neighbor);
            }
        }
    }
    groups
        .into_labeling()
        .into_iter()
        .collect::<HashSet<_>>()
        .len()
}

fn get_filled<'a>(input: &'a str) -> impl Iterator<Item = Vector> + 'a {
    (0..128).flat_map(move |y| {
        let key = input.to_owned() + "-" + &y.to_string();
        KnotHash::from(key.as_ref())
            .as_bits()
            .into_iter()
            .enumerate()
            .filter(|&(_, b)| b)
            .map(move |(x, _)| Vector(x as i32, y as i32))
    })
}
