use std::collections::HashMap;

type Int = i64;

pub fn solve(input: &str) -> (i64, i64) {
    let mut registers = HashMap::<&str, Int>::new();
    let mut max = Int::min_value();
    for instruction in input.lines().map(parse_line) {
        let reg = *registers.get(instruction.register).unwrap_or(&0);
        let value = instruction.value;
        if match instruction.comparator {
            ">" => reg > value,
            "<" => reg < value,
            ">=" => reg >= value,
            "<=" => reg <= value,
            "==" => reg == value,
            "!=" => reg != value,
            _ => unreachable!(),
        } {
            let target = registers.entry(instruction.target).or_insert(0);
            *target += instruction.shift;
            max = Ord::max(max, *target);
        }
    }
    (*registers.values().max().unwrap(), max)
}

#[derive(Debug)]
struct Instruction<'a> {
    target: &'a str,
    shift: Int,
    register: &'a str,
    comparator: &'a str,
    value: Int,
}

fn parse_line(line: &str) -> Instruction {
    let split = line.split_whitespace().collect::<Vec<_>>();
    match &split[..] {
        &[target, direction, offset, "if", register, comparator, value] => {
            let sign = match direction {
                "dec" => -1,
                "inc" => 1,
                _ => unreachable!(),
            };
            let shift = offset.parse::<Int>().unwrap() * sign;
            let value = value.parse().unwrap();
            Instruction {
                target,
                shift,
                register,
                comparator,
                value,
            }
        }
        _ => unreachable!(),
    }
}
