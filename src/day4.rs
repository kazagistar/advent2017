use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    input.lines().filter(|pw| !dupes(pw)).count()
}

fn dupes(passphrase: &str) -> bool {
    let mut dupes = HashSet::new();
    for password in passphrase.split_whitespace() {
        if dupes.contains(password) {
            return true;
        }
        dupes.insert(password);
    }
    return false;
}

pub fn part2(input: &str) -> usize {
    input.lines().filter(|pw| !anagram_dupes(pw)).count()
}

fn anagram_dupes(passphrase: &str) -> bool {
    let mut dupes = HashSet::new();
    for password in passphrase.split_whitespace() {
        let mut password: Vec<_> = password.chars().collect();
        password.sort();
        if dupes.contains(&password) {
            return true;
        }
        dupes.insert(password);
    }
    return false;
}