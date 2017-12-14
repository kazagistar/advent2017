use day10::KnotHash;
use std::collections::{HashMap, HashSet};
use util::Vector;
use petgraph::unionfind::UnionFind;

pub fn part1(input: &str) -> usize {
    (0..128)
        .map(|i| {
            let key = input.to_owned() + "-" + &i.to_string();
            let row = KnotHash::from(key.as_ref()).as_bin();
            row.chars().filter(|&c| c == '1').count()
        })
        .sum()
}

static AROUND: &'static [Vector] = &[Vector(0, 1), Vector(1, 0), Vector(0, -1), Vector(-1, 0)];

pub fn part2(input: &str) -> usize {
    let disk: HashMap<Vector, usize> = (0..128)
        .flat_map(|y| {
            let key = input.to_owned() + "-" + &y.to_string();
            let row: Vec<_> = KnotHash::from(key.as_ref()).as_bin().chars().collect();
            row.into_iter().enumerate().filter_map(move |(x, c)| {
                if c == '1' {
                    Some(Vector(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .zip(0..)
        .collect();
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
