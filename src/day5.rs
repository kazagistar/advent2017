pub fn part1(input: &str) -> usize {
    escape(input, |x| x + 1)
}

pub fn part2(input: &str) -> usize {
    escape(input, |x| if x >= 3 { x - 1 } else { x + 1 })
}

fn escape(input: &str, update: impl Fn(i32) -> i32) -> usize {
    let mut maze: Vec<i32> = input
        .split_whitespace()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut position = 0;
    let mut count = 0;
    while (0..maze.len() as i32).contains(position) {
        let offset = maze[position as usize];
        maze[position as usize] = update(offset);
        position += offset;
        count += 1
    }
    count
}

#[test]
fn examples() {
    let input = "0 3 0 1 -3";
    assert_eq!(5, part1(input));
    assert_eq!(10, part2(input));
}
