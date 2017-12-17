pub fn part1(input: usize) -> u16 {
    let mut b = Vec::with_capacity(2018);
    b.push(0);
    let mut i = 0;
    for year in 1..=2017 {
        i = (i + input + 1) % b.len();
        b.insert(i, year);
    }
    b[(i + 1) % b.len()]
}

pub fn part2(input: usize) -> usize {
    let mut after_0 = 0;
    let mut i = 0;
    for year in 1..=50000000 {
        i = (i + input + 1) % year;
        if i == 0 {
            after_0 = year
        }
    }
    after_0
}
