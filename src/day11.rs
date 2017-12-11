use util::Vector;

fn step(direction: &str) -> Vector {
    match direction {
        "n" => Vector(0, 1),
        "nw" => Vector(-1, 0),
        "sw" => Vector(-1, -1),
        "s" => Vector(0, -1),
        "se" => Vector(1, 0),
        "ne" => Vector(1, 1),
        _ => panic!("Bad input"),
    }
}

fn hex_distance(&Vector(x, y): &Vector) -> i32 {
    if x * y < 0 {
        x.abs() + y.abs()
    } else {
        x.abs().max(y.abs())
    }
}

pub fn part1(input: &str) -> i32 {
    hex_distance(&input.split(",").map(step).sum())
}

pub fn part2(input: &str) -> i32 {
    let mut pos = Vector(0, 0);
    let mut furthest = 0;
    for vec in input.split(",").map(step) {
        pos += vec;
        furthest = furthest.max(hex_distance(&pos));
    }
    furthest
}

#[test]
fn examples() {
    assert_eq!(3, part1("ne,ne,ne"));
    assert_eq!(0, part1("ne,ne,sw,sw"));
    assert_eq!(2, part1("ne,ne,s,s"));
    assert_eq!(3, part1("se,sw,se,sw,sw"));

    assert_eq!(2, part2("ne,ne,sw,sw"));
}
