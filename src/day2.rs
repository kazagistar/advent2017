fn parser(input: &str) -> Vec<Vec<i32>> {
    input
        .split('\n')
        .map(|line| {
            line.split('\t')
                .flat_map(|cell| cell.parse())
                .collect::<Vec<_>>()
        })
        .filter(|row| row.len() != 0)
        .collect()
}

pub fn part1(input: &str) -> i32 {
    parser(input)
        .iter()
        .map(|v| v.iter().max().unwrap() - v.iter().min().unwrap())
        .sum()
}

fn combinations(input: &[i32]) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    let mut iter = input.iter().cloned();
    while let Some(x) = iter.next() {
        for y in iter.clone() {
            result.push((x, y));
        }
    }
    result
}

pub fn part2(input: &str) -> i32 {
    parser(input)
        .iter()
        .map(|v| {
            combinations(&v)
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
