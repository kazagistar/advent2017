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

fn count_uniques<'a, Key: Ord + Hash + 'a>(input: &'a str, key: impl Fn(&'a str) -> Key) -> usize {
    input
        .lines()
        .filter(|passphrase| {
            let mut dupes = HashSet::new();
            passphrase
                .split_whitespace()
                .flat_map(|word| dupes.replace(key(word)))
                .count() == 0
        })
        .count()
}

#[test]
fn examples() {
    assert_eq!(1, part1("aa bb cc dd ee"));
    assert_eq!(0, part1("aa bb cc dd aa"));
    assert_eq!(1, part1("aa bb cc dd aaa"));

    assert_eq!(1, part2("abcde fghij"));
    assert_eq!(0, part2("abcde xyz ecdab"));
    assert_eq!(1, part2("a ab abc abd abf abj"));
    assert_eq!(1, part2("iiii oiii ooii oooi oooo"));
    assert_eq!(0, part2("oiii ioii iioi iiio"));
}
