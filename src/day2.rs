fn mapsum_lines(input: &str, map: impl Fn(&[i32]) -> i32) -> i32 {
    input
        .lines()
        .map(|line| {
            map(&line.split_whitespace()
                .map(|word| word.parse().unwrap())
                .collect::<Vec<_>>())
        })
        .sum()
}

pub fn part1(input: &str) -> i32 {
    mapsum_lines(input, |v| v.iter().max().unwrap() - v.iter().min().unwrap())
}

fn combinations<'a>(mut iter: impl Iterator<Item = i32> + Clone + 'a) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    while let Some(x) = iter.next() {
        for y in iter.clone() {
            result.push((x, y));
        }
    }
    result
}

pub fn part2(input: &str) -> i32 {
    mapsum_lines(input, |v| {
        combinations(v.iter().cloned())
            .iter()
            .map(|&(x, y)| {
                if x % y == 0 {
                    x / y
                } else if y % x == 0 {
                    y / x
                } else {
                    0
                }
            })
            .sum::<i32>()
    })
}

#[test]
fn examples() {
    let sheet1 = "5 1 9 5
                  7 5 3
                  2 4 6 8";
    assert_eq!(18, part1(sheet1));

    let sheet2 = "5 9 2 8
                  9 4 7 3
                  3 8 6 5";
    assert_eq!(9, part2(sheet2));
}
