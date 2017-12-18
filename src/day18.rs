use self::Mem::*;
use self::Event::*;
use std::collections::{HashMap, VecDeque};

type Int = i64;

#[derive(Debug, Copy, Clone)]
enum Mem {
    Num(Int),
    Reg(char),
}

fn parse_mem(mem: &str) -> Mem {
    mem.parse()
        .map(Num)
        .unwrap_or_else(|_| Reg(mem.chars().next().unwrap()))
}

#[derive(Debug, Clone)]
struct Cmd<'a>(&'a str, Mem, Mem);

fn parse_program(program: &str) -> Vec<Cmd> {
    program
        .lines()
        .map(|line| {
            let mut items = line.trim().split_whitespace();
            Cmd(
                items.next().unwrap(),
                parse_mem(items.next().unwrap()),
                items.next().map(parse_mem).unwrap_or_else(|| Reg(' ')),
            )
        })
        .collect()
}

#[derive(Debug, PartialEq)]
enum Event {
    Snd(Int),
    Rcv(char),
    End,
}

#[derive(Debug)]
struct Machine<'a> {
    program: Vec<Cmd<'a>>,
    registers: HashMap<char, Int>,
    pc: usize,
}

impl<'a> Machine<'a> {
    fn new(program: Vec<Cmd<'a>>) -> Self {
        let mut registers = HashMap::new();
        for &Cmd(_, x, y) in &program {
            if let Reg(c) = x {
                registers.insert(c, 0);
            }
            if let Reg(c) = y {
                registers.insert(c, 0);
            }
        }
        Machine {
            program,
            registers,
            pc: 0,
        }
    }

    fn get(&self, mem: Mem) -> Int {
        match mem {
            Num(i) => i,
            Reg(r) => self.registers.get(&r).map(|i| *i).unwrap_or(0),
        }
    }

    fn run(&mut self) -> Event {
        let mut status = None;
        while status == None {
            match &self.program[self.pc] {
                &Cmd("set", Reg(r), x) => *self.registers.entry(r).or_insert(0) = self.get(x),
                &Cmd("add", Reg(r), x) => *self.registers.entry(r).or_insert(0) += self.get(x),
                &Cmd("mul", Reg(r), x) => *self.registers.entry(r).or_insert(0) *= self.get(x),
                &Cmd("mod", Reg(r), x) => *self.registers.entry(r).or_insert(0) %= self.get(x),
                &Cmd("jgz", x, y) => if self.get(x) > 0 {
                    self.pc = (self.pc as Int + self.get(y) - 1) as usize
                },
                &Cmd("snd", x, _) => status = Some(Snd(self.get(x))),
                &Cmd("rcv", Reg(r), _) => status = Some(Rcv(r)),
                c => panic!("Bad command: {:?}", c),
            }
            self.pc += 1;
            if !(0..self.program.len()).contains(self.pc) {
                status = Some(End)
            }
        }
        status.unwrap()
    }
}

pub fn part1(input: &str) -> i64 {
    let mut machine = Machine::new(parse_program(input));
    let mut sent = None;
    loop {
        match machine.run() {
            Snd(x) => {
                sent = Some(x);
            }
            Rcv(r) => if machine.registers[&r] != 0 {
                return sent.unwrap();
            },
            End => {
                panic!("Program counter escaped before rcv was called!");
            }
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum State {
    Ready,
    Done,
    Blocked(char),
}
use self::State::*;

#[derive(Debug)]
struct CoMachine<'a> {
    inner: Machine<'a>,
    state: State,
    queue: VecDeque<Int>,
}

impl<'a> CoMachine<'a> {
    fn new(program: Vec<Cmd<'a>>, id: Int) -> Self {
        let mut inner = Machine::new(program);
        inner.registers.insert('p', id);
        CoMachine {
            inner,
            state: Ready,
            queue: VecDeque::new(),
        }
    }

    fn try_feed(&mut self) {
        if let Blocked(r) = self.state {
            if let Some(x) = self.queue.pop_front() {
                self.inner.registers.insert(r, x);
                self.state = Ready;
            }
        }
    }

    fn run(&mut self, target: &mut VecDeque<Int>) {
        self.try_feed();
        while self.state == Ready {
            match self.inner.run() {
                Snd(i) => target.push_back(i),
                Rcv(r) => {
                    self.state = Blocked(r);
                    self.try_feed();
                }
                End => self.state = Done,
            }
        }
    }

    fn halted(&self) -> bool {
        match self.state {
            Ready => false,
            Done => true,
            Blocked(_) => self.queue.is_empty(),
        }
    }
}

pub fn part2(input: &str) -> usize {
    let program = parse_program(input);
    let mut m0 = CoMachine::new(program.clone(), 0);
    let mut m1 = CoMachine::new(program, 1);
    let mut sent_by_m1 = 0;
    while !(m0.halted() && m1.halted()) {
        m0.run(&mut m1.queue);
        m1.run(&mut m0.queue);
        sent_by_m1 += m0.queue.len();
    }
    sent_by_m1
}

#[test]
fn examples() {
    let input1 = "set a 1
                  add a 2
                  mul a a
                  mod a 5
                  snd a
                  set a 0
                  rcv a
                  jgz a -1
                  set a 1
                  jgz a -2";
    assert_eq!(4, part1(input1));

    let input2 = "snd 1
                  snd 2
                  snd p
                  rcv a
                  rcv b
                  rcv c
                  rcv d";
    assert_eq!(3, part2(input2));
}
