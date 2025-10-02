use std::collections::VecDeque;
use std::num::ParseIntError;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
    #[error("Invalid register name")]
    InvalidRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Reg {
    A,
    B,
    C,
    D,
    F,
    I,
    P,
}

impl Reg {
    const fn all() -> [Self; 7] {
        [Self::A, Self::B, Self::C, Self::D, Self::F, Self::I, Self::P]
    }

    const fn new(ch: u8) -> Result<Self, ParseError> {
        Ok(match ch {
            b'a' => Self::A,
            b'b' => Self::B,
            b'c' => Self::C,
            b'd' => Self::D,
            b'f' => Self::F,
            b'i' => Self::I,
            b'p' => Self::P,
            _ => return Err(ParseError::InvalidRegister),
        })
    }
}

impl FromStr for Reg {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let &[ch] = s.as_bytes() {
            Self::new(ch)
        } else {
            Err(ParseError::InvalidRegister)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegOrValue {
    Reg(Reg),
    Value(i64),
}

impl FromStr for RegOrValue {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.as_bytes() {
            &[b'-' | b'0'..=b'9', ..] => Self::Value(s.parse()?),
            _ => Self::Reg(s.parse()?),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BinOp {
    Set,
    Add,
    Mul,
    Mod,
}

impl BinOp {
    const fn apply(self, target: &mut i64, rhs: i64) {
        match self {
            Self::Set => *target = rhs,
            Self::Add => *target = target.checked_add(rhs).expect("overflow"),
            Self::Mul => *target = target.checked_mul(rhs).expect("overflow"),
            Self::Mod => *target = target.checked_rem(rhs).expect("overflow"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Snd(RegOrValue),
    BinOp(BinOp, Reg, RegOrValue),
    Rcv(Reg),
    Jgz(RegOrValue, RegOrValue),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(rest) = s.strip_prefix("snd ") {
            Self::Snd(rest.parse()?)
        } else if let Some(rest) = s.strip_prefix("set ") {
            let (reg, value) = rest.split_once(' ').ok_or(ParseError::SyntaxError)?;
            Self::BinOp(BinOp::Set, reg.parse()?, value.parse()?)
        } else if let Some(rest) = s.strip_prefix("add ") {
            let (reg, value) = rest.split_once(' ').ok_or(ParseError::SyntaxError)?;
            Self::BinOp(BinOp::Add, reg.parse()?, value.parse()?)
        } else if let Some(rest) = s.strip_prefix("mul ") {
            let (reg, value) = rest.split_once(' ').ok_or(ParseError::SyntaxError)?;
            Self::BinOp(BinOp::Mul, reg.parse()?, value.parse()?)
        } else if let Some(rest) = s.strip_prefix("mod ") {
            let (reg, value) = rest.split_once(' ').ok_or(ParseError::SyntaxError)?;
            Self::BinOp(BinOp::Mod, reg.parse()?, value.parse()?)
        } else if let Some(rest) = s.strip_prefix("rcv ") {
            Self::Rcv(rest.parse()?)
        } else if let Some(rest) = s.strip_prefix("jgz ") {
            let (check, delta) = rest.split_once(' ').ok_or(ParseError::SyntaxError)?;
            Self::Jgz(check.parse()?, delta.parse()?)
        } else {
            return Err(ParseError::SyntaxError);
        })
    }
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Result<Vec<Instruction>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day18, part1)]
fn part_1(instructions: &[Instruction]) -> i64 {
    let mut machine = Machine::new(instructions, true);
    machine.run();
    machine.output_queue.pop_back().unwrap_or(0)
}

#[aoc(day18, part2)]
fn part_2(instructions: &[Instruction]) -> usize {
    let reg_p = Reg::new(b'p').unwrap();
    let mut machine0 = Machine::new(instructions, false);
    machine0[reg_p] = 0;
    machine0.run();
    let mut machine1 = Machine::new(instructions, false);
    machine1[reg_p] = 1;
    machine1.run();
    loop {
        if !machine0.output_queue.is_empty() {
            machine1.input_queue.extend(machine0.output_queue.drain(..));
        }
        if !machine1.output_queue.is_empty() {
            machine0.input_queue.extend(machine1.output_queue.drain(..));
        }
        if machine0.state == State::WaitingForInput && !machine0.input_queue.is_empty() {
            machine0.run();
        } else if machine1.state == State::WaitingForInput && !machine1.input_queue.is_empty() {
            machine1.run();
        } else {
            return machine1.output_count;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Pending,
    WaitingForInput,
    Stopped,
}

#[derive(Debug, Clone)]
struct Machine<'a> {
    instructions: &'a [Instruction],
    rcv_nonzero: bool,
    state: State,
    ip: usize,
    registers: [i64; Reg::all().len()],
    output_queue: VecDeque<i64>,
    output_count: usize,
    input_queue: VecDeque<i64>,
}

impl<'a> Machine<'a> {
    pub const fn new(instructions: &'a [Instruction], rcv_nonzero: bool) -> Self {
        Self {
            instructions,
            rcv_nonzero,
            state: State::Pending,
            ip: 0,
            registers: [0; Reg::all().len()],
            output_queue: VecDeque::new(),
            output_count: 0,
            input_queue: VecDeque::new(),
        }
    }

    fn get_value(&self, source: RegOrValue) -> i64 {
        match source {
            RegOrValue::Reg(reg) => self[reg],
            RegOrValue::Value(val) => val,
        }
    }

    fn step(&mut self) {
        if self.state == State::WaitingForInput && !self.input_queue.is_empty() {
            self.state = State::Pending;
        }
        if self.state != State::Pending {
            return;
        }
        let Some(&instr) = self.instructions.get(self.ip) else {
            self.state = State::Stopped;
            return;
        };
        match instr {
            Instruction::Snd(src) => {
                self.output_count += 1;
                self.output_queue.push_back(self.get_value(src));
            }
            Instruction::BinOp(op, reg, rhs) => {
                let rhs = self.get_value(rhs);
                op.apply(&mut self[reg], rhs);
            }
            Instruction::Rcv(reg) => {
                if !self.rcv_nonzero || self[reg] != 0 {
                    if let Some(rcv_value) = self.input_queue.pop_front() {
                        self[reg] = rcv_value;
                    } else {
                        self.state = State::WaitingForInput;
                        return;
                    }
                }
            }
            Instruction::Jgz(check, delta) => {
                if self.get_value(check) > 0 {
                    if let Some(new_ip) = self
                        .ip
                        .checked_add_signed(isize::try_from(self.get_value(delta)).unwrap())
                        && new_ip < self.instructions.len()
                    {
                        self.ip = new_ip;
                        return;
                    }
                    self.state = State::Stopped;
                }
            }
        }
        self.ip += 1;
    }

    fn run(&mut self) {
        if self.state == State::WaitingForInput && !self.input_queue.is_empty() {
            self.state = State::Pending;
        }
        while self.state == State::Pending {
            self.step();
        }
    }
}

impl Index<Reg> for Machine<'_> {
    type Output = i64;

    fn index(&self, reg: Reg) -> &Self::Output {
        &self.registers[reg as usize]
    }
}

impl IndexMut<Reg> for Machine<'_> {
    fn index_mut(&mut self, reg: Reg) -> &mut Self::Output {
        &mut self.registers[reg as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
        set a 1\n\
        add a 2\n\
        mul a a\n\
        mod a 5\n\
        snd a\n\
        set a 0\n\
        rcv a\n\
        jgz a -1\n\
        set a 1\n\
        jgz a -2\
        ";

    const EXAMPLE2: &str = "\
        snd 1\n\
        snd 2\n\
        snd p\n\
        rcv a\n\
        rcv b\n\
        rcv c\n\
        rcv d\
        ";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE1).unwrap();
        let reg_a = Reg::new(b'a').unwrap();
        assert_eq!(
            result,
            [
                Instruction::BinOp(BinOp::Set, reg_a, RegOrValue::Value(1)),
                Instruction::BinOp(BinOp::Add, reg_a, RegOrValue::Value(2)),
                Instruction::BinOp(BinOp::Mul, reg_a, RegOrValue::Reg(reg_a)),
                Instruction::BinOp(BinOp::Mod, reg_a, RegOrValue::Value(5)),
                Instruction::Snd(RegOrValue::Reg(reg_a)),
                Instruction::BinOp(BinOp::Set, reg_a, RegOrValue::Value(0)),
                Instruction::Rcv(reg_a),
                Instruction::Jgz(RegOrValue::Reg(reg_a), RegOrValue::Value(-1)),
                Instruction::BinOp(BinOp::Set, reg_a, RegOrValue::Value(1)),
                Instruction::Jgz(RegOrValue::Reg(reg_a), RegOrValue::Value(-2))
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let instructions = parse(EXAMPLE1).unwrap();
        let result = part_1(&instructions);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_2() {
        let instructions = parse(EXAMPLE2).unwrap();
        let result = part_2(&instructions);
        assert_eq!(result, 3);
    }
}
