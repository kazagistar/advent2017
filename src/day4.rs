use std::collections::HashSet;
use std::hash::Hash;

pub fn part1(input: &str) -> usize {
    count_uniques(input, |x| x)
}

pub fn part2(input: &str) -> usize {
    count_uniques(input, |word| {
        let mut word: Vec<_> = word.chars().collect();
        word.sort();
        word
    })
}

fn count_uniques<'a, Key: Ord + Hash + Eq + 'a>(input: &'a str, key: impl Fn(&'a str) -> Key) -> usize {
    input.lines().filter(|passphrase| {
        let mut dupes = HashSet::new();
        passphrase.split_whitespace().flat_map(|word| dupes.replace(key(word))).count() == 0
    }).count()
}
