use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Eq, Debug)]
struct Record {
    record: Vec<u32>,
    index: usize,
}

impl Hash for Record {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.record.hash(state);
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.record == other.record
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut state: Vec<u32> = input
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();
    let mut history = HashSet::new();
    for counter in 0.. {
        let old = state.clone();
        if let Some(repeat) = history.replace(Record {
            record: old,
            index: counter,
        }) {
            return (counter, counter - repeat.index);
        }
        let (mut bank, spare) = state
            .iter_mut()
            .enumerate()
            .max_by_key(|&(index, &mut memory)| (memory, 0 - (index as i32)))
            .map(|(index, memory)| {
                let mem = memory.clone();
                *memory = 0;
                (index, mem)
            })
            .unwrap();
        for _ in 0..spare {
            bank = (bank + 1) % state.len();
            state[bank] += 1;
        }
    }
    unreachable!()
}

#[test]
fn examples() {
    assert_eq!((5, 4), solve("0 2 7 0"));
}
