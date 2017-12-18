use std::collections::{HashMap, VecDeque};

type Int = i64;
type Reg = char;

#[derive(Debug, Copy, Clone)]
enum Mem {
    Number(Int),
    Register(Reg),
}
use self::Mem::*;

fn parse_mem(mem: &str) -> Mem {
    mem.parse()
        .map(Number)
        .unwrap_or_else(|_| Register(mem.chars().next().unwrap()))
}

#[derive(Debug)]
struct Cmd<'a>(&'a str, Mem, Mem);

fn parse_program(source: &str) -> Vec<Cmd> {
    source
        .lines()
        .map(|line| {
            let mut items = line.trim().split_whitespace();
            Cmd(
                items.next().unwrap(),
                parse_mem(items.next().unwrap()),
                items.next().map(parse_mem).unwrap_or_else(|| Register(' ')),
            )
        })
        .collect()
}

#[derive(Debug, PartialEq)]
enum Event {
    Snd(Int),
    Rcv(Reg),
    End,
}
use self::Event::*;

#[derive(Debug)]
struct Machine<'a> {
    program: &'a [Cmd<'a>],
    registers: HashMap<Reg, Int>,
    pc: usize,
}

impl<'a> Machine<'a> {
    fn new(program: &'a [Cmd<'a>]) -> Self {
        let mut registers = HashMap::new();
        for &Cmd(_, x, y) in program {
            if let Register(c) = x {
                registers.insert(c, 0);
            }
            if let Register(c) = y {
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
            Number(i) => i,
            Register(r) => self.registers.get(&r).map(|i| *i).unwrap_or(0),
        }
    }

    fn modify(&mut self, r: Reg, x: Mem, f: impl Fn(&mut Int, Int)) {
        let v = self.get(x);
        f(self.registers.get_mut(&r).unwrap(), v);
    }

    fn run(&mut self) -> Event {
        let mut status = None;
        while status == None {
            match &self.program[self.pc] {
                &Cmd("set", Register(r), x) => self.modify(r, x, |r, x| *r = x),
                &Cmd("add", Register(r), x) => self.modify(r, x, |r, x| *r += x),
                &Cmd("mul", Register(r), x) => self.modify(r, x, |r, x| *r *= x),
                &Cmd("mod", Register(r), x) => self.modify(r, x, |r, x| *r %= x),
                &Cmd("jgz", x, y) => if self.get(x) > 0 {
                    self.pc = (self.pc as Int + self.get(y) - 1) as usize
                },
                &Cmd("snd", x, _) => status = Some(Snd(self.get(x))),
                &Cmd("rcv", Register(r), _) => status = Some(Rcv(r)),
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
    let program = parse_program(input);
    let mut machine = Machine::new(&program);
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

#[derive(Debug, PartialEq)]
enum State {
    Ready,
    Done,
    Blocked(Reg),
}
use self::State::*;

#[derive(Debug)]
struct CoMachine<'a> {
    inner: Machine<'a>,
    state: State,
    queue: VecDeque<Int>,
}

impl<'a> CoMachine<'a> {
    fn new(program: &'a [Cmd<'a>], id: Int) -> Self {
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
    let mut m0 = CoMachine::new(&program, 0);
    let mut m1 = CoMachine::new(&program, 1);
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
