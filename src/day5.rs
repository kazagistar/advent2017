pub fn part1(input: &str) -> usize {
    let mut maze: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut pos = 0;
    let mut count = 0;
    while 0 <= pos && pos < maze.len() as i32 {
        let offset = maze[pos as usize];
        maze[pos as usize] += 1;
        pos += offset;
        count += 1
    }
    count
}

pub fn part2(input: &str) -> usize {
    let mut maze: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut pos = 0;
    let mut count = 0;
    while 0 <= pos && pos < maze.len() as i32 {
        let offset = maze[pos as usize];
        maze[pos as usize] += if offset >= 3 { -1 } else { 1 };
        pos += offset;
        count += 1
    }
    count
}
