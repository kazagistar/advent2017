use std::collections::{BinaryHeap, HashSet};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Clone)]
struct Bank {
    memory: u16,
    index: u16,
}

impl Bank {
    fn take_memory(&mut self) -> u16 {
        let mem = self.memory;
        self.memory = 0;
        mem
    }
}

pub fn part1(input: &str) -> usize {
    let mut state: Vec<_> = input
        .split_whitespace()
        .enumerate()
        .map(|(index, memory)| {
            Bank {
                index: index as u16,
                memory: memory.parse().unwrap(),
            }
        })
        .collect();
    let mut history = HashSet::new();
    for counter in 0.. {
        state.sort();
        let old = state.clone();
        println!("{} = {:?}", counter, old);
        if let Some(_) = history.replace(old) {
            return counter;
        }
        let mut spare: u16 = state.last_mut().unwrap().take_memory();
        state.sort();
        'rebalance: loop {
            for bank in state.iter_mut() {
                if spare == 0 {
                    break 'rebalance;
                }
                bank.memory += 1;
                spare -= 1;
            }
        }
    }
    unreachable!()
}

#[test]
fn day6test() {
    assert_eq!(5, part1("0 2 7 0"));
}
