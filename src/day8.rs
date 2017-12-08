use std::collections::HashMap;

type Int = i64;

pub fn solve(input: &str) -> (i64, i64) {
    let mut registers = HashMap::<&str, Int>::new();
    let mut max = Int::min_value();
    for inst in input.lines().map(parse_line) {
        let reg = *registers.get(inst.reg).unwrap_or(&0);
        let cond = match inst.cmp {
            ">" => reg > inst.val,
            "<" => reg < inst.val,
            ">=" => reg >= inst.val,
            "<=" => reg <= inst.val,
            "==" => reg == inst.val,
            "!=" => reg != inst.val,
            _ => unreachable!(),
        };
        if cond {
            let target = registers.entry(inst.target).or_insert(0);
            *target += inst.shift;
            max = Ord::max(max, *target);
        }
    }
    (*registers.values().max().unwrap(), max)
}

#[derive(Debug)]
struct Instruction<'a> {
    target: &'a str,
    shift: Int,
    reg: &'a str,
    cmp: &'a str,
    val: Int,
}

fn parse_line(line: &str) -> Instruction {
    let items: Vec<&str> = line.split_whitespace().collect();
    let sign = match items[1] {
        "dec" => -1,
        "inc" => 1,
        _ => unreachable!(),
    };
    let shift_base: Int = items[2].parse().unwrap();
    Instruction {
        target: items[0],
        shift: shift_base * sign,
        reg: items[4],
        cmp: items[5],
        val: items[6].parse().unwrap(),
    }
}
