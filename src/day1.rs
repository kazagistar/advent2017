// Zip togeather the items with a given offset, and if they are equal, sum them
pub fn sum_matching(input: &str, offset: usize) -> u32 {
    input
        .chars()
        .zip(input.chars().cycle().skip(offset))
        .filter(|&(a, b)| a == b)
        .map(|x| x.1.to_digit(10).unwrap())
        .sum()
}

pub fn part1(input: &str) -> u32 {
    sum_matching(input, 1)
}

pub fn part2(input: &str) -> u32 {
    sum_matching(input, input.len() / 2)
}

#[test]
fn examples() {
    assert_eq!(3, part1("1122"));
    assert_eq!(4, part1("1111"));
    assert_eq!(0, part1("1234"));
    assert_eq!(9, part1("91212129"));

    assert_eq!(6, part2("1212"));
    assert_eq!(0, part2("1221"));
    assert_eq!(4, part2("123425"));
    assert_eq!(12, part2("123123"));
    assert_eq!(4, part2("12131415"));
}
