use input::spreadsheet_parse as parse;

pub fn part1(input: &str) -> i32 {
    parse(input)
        .map(|v| v.clone().max().unwrap() - v.min().unwrap())
        .sum()
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
    parse(input)
        .map(|v| {
            combinations(v)
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
        .sum()
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
