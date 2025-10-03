use std::{
    fmt::Display,
    num::ParseIntError,
    ops::{Index, IndexMut},
    str::FromStr,
};

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
    E,
    F,
    G,
    H,
}

impl Reg {
    const fn all() -> [Self; 8] {
        [
            Self::A,
            Self::B,
            Self::C,
            Self::D,
            Self::E,
            Self::F,
            Self::G,
            Self::H,
        ]
    }

    const fn new(ch: u8) -> Result<Self, ParseError> {
        Ok(match ch {
            b'a' => Self::A,
            b'b' => Self::B,
            b'c' => Self::C,
            b'd' => Self::D,
            b'e' => Self::E,
            b'f' => Self::F,
            b'g' => Self::G,
            b'h' => Self::H,
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

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => f.write_str("a"),
            Self::B => f.write_str("b"),
            Self::C => f.write_str("c"),
            Self::D => f.write_str("d"),
            Self::E => f.write_str("e"),
            Self::F => f.write_str("f"),
            Self::G => f.write_str("g"),
            Self::H => f.write_str("h"),
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

impl Display for RegOrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reg(reg) => reg.fmt(f),
            Self::Value(v) => v.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BinOp {
    Set,
    Sub,
    Mul,
    Mod,
}

impl BinOp {
    const fn apply(self, target: &mut i64, rhs: i64) {
        match self {
            Self::Set => *target = rhs,
            Self::Sub => *target = target.checked_sub(rhs).expect("overflow"),
            Self::Mul => *target = target.checked_mul(rhs).expect("overflow"),
            Self::Mod => *target = target.checked_rem(rhs).expect("overflow"),
        }
    }
}

impl FromStr for BinOp {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "set" => Self::Set,
            "sub" => Self::Sub,
            "mul" => Self::Mul,
            "mod" => Self::Mod,
            _ => return Err(ParseError::SyntaxError),
        })
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Set => f.write_str("set"),
            Self::Sub => f.write_str("sub"),
            Self::Mul => f.write_str("mul"),
            Self::Mod => f.write_str("mod"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    BinOp(BinOp, Reg, RegOrValue),
    Jnz(RegOrValue, RegOrValue),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');
        Ok(match words.next().ok_or(ParseError::SyntaxError)? {
            "jnz" => Self::Jnz(
                words.next().ok_or(ParseError::SyntaxError)?.parse()?,
                words.next().ok_or(ParseError::SyntaxError)?.parse()?,
            ),
            op @ ("set" | "sub" | "mul" | "mod") => Self::BinOp(
                op.parse()?,
                words.next().ok_or(ParseError::SyntaxError)?.parse()?,
                words.next().ok_or(ParseError::SyntaxError)?.parse()?,
            ),
            _ => return Err(ParseError::SyntaxError),
        })
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BinOp(op, reg, value) => write!(f, "{op} {reg} {value}"),
            Self::Jnz(cond, delta) => write!(f, "jnz {cond} {delta}"),
        }
    }
}

#[aoc_generator(day23)]
fn parse(input: &str) -> Result<Vec<Instruction>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day23, part1)]
fn part_1(instructions: &[Instruction]) -> usize {
    let mut machine = Machine::new(instructions);
    machine.run();
    machine.mul_count
}

#[aoc(day23, part2)]
fn part_2(instructions: &[Instruction]) -> i64 {
    let optimized = optimize(instructions);
    let mut machine = Machine::new(&optimized);
    machine[Reg::A] = 1;
    machine.run();
    machine[Reg::H]
}

fn optimize(instructions: &[Instruction]) -> Vec<Instruction> {
    let target = [
        Instruction::BinOp(BinOp::Set, Reg::E, RegOrValue::Value(2)),
        Instruction::BinOp(BinOp::Set, Reg::G, RegOrValue::Reg(Reg::D)),
        Instruction::BinOp(BinOp::Mul, Reg::G, RegOrValue::Reg(Reg::E)),
        Instruction::BinOp(BinOp::Sub, Reg::G, RegOrValue::Reg(Reg::B)),
        Instruction::Jnz(RegOrValue::Reg(Reg::G), RegOrValue::Value(2)),
        Instruction::BinOp(BinOp::Set, Reg::F, RegOrValue::Value(0)),
        Instruction::BinOp(BinOp::Sub, Reg::E, RegOrValue::Value(-1)),
        Instruction::BinOp(BinOp::Set, Reg::G, RegOrValue::Reg(Reg::E)),
        Instruction::BinOp(BinOp::Sub, Reg::G, RegOrValue::Reg(Reg::B)),
        Instruction::Jnz(RegOrValue::Reg(Reg::G), RegOrValue::Value(-8)),
        Instruction::BinOp(BinOp::Sub, Reg::D, RegOrValue::Value(-1)),
        Instruction::BinOp(BinOp::Set, Reg::G, RegOrValue::Reg(Reg::D)),
        Instruction::BinOp(BinOp::Sub, Reg::G, RegOrValue::Reg(Reg::B)),
        Instruction::Jnz(RegOrValue::Reg(Reg::G), RegOrValue::Value(-13)),
    ];
    let replacement = [
        Instruction::BinOp(BinOp::Set, Reg::G, RegOrValue::Reg(Reg::B)),
        Instruction::BinOp(BinOp::Mod, Reg::G, RegOrValue::Reg(Reg::D)),
        Instruction::Jnz(RegOrValue::Reg(Reg::G), RegOrValue::Value(3)),
        Instruction::BinOp(BinOp::Set, Reg::F, RegOrValue::Value(0)),
        Instruction::Jnz(RegOrValue::Value(1), RegOrValue::Value(4)),
        Instruction::BinOp(BinOp::Sub, Reg::D, RegOrValue::Value(-1)),
        Instruction::BinOp(BinOp::Set, Reg::G, RegOrValue::Reg(Reg::D)),
        Instruction::BinOp(BinOp::Sub, Reg::G, RegOrValue::Reg(Reg::B)),
        Instruction::Jnz(RegOrValue::Reg(Reg::G), RegOrValue::Value(-8)),
    ];
    let mut result = Vec::with_capacity(instructions.len() - target.len() + replacement.len());
    for i in 0..instructions.len() - target.len() {
        let Some(mapping) = extract_register_mapping(&instructions[i..], &target) else {
            continue;
        };
        result.clear();
        for (j, &(mut before)) in instructions[..i].iter().enumerate() {
            if let Instruction::Jnz(_, RegOrValue::Value(ref mut v)) = before
                && j.saturating_add_signed(isize::try_from(*v).unwrap()) >= i + target.len()
            {
                *v += i64::try_from(replacement.len()).unwrap()
                    - i64::try_from(target.len()).unwrap();
            }
            result.push(before);
        }
        for &instr in &replacement {
            result.push(match instr {
                Instruction::BinOp(op, reg, val) => Instruction::BinOp(
                    op,
                    mapping.reverse_reg(reg).unwrap(),
                    mapping.reverse_reg_or_value(val).unwrap(),
                ),
                Instruction::Jnz(cond, delta) => Instruction::Jnz(
                    mapping.reverse_reg_or_value(cond).unwrap(),
                    mapping.reverse_reg_or_value(delta).unwrap(),
                ),
            });
        }
        for (&(mut before), j) in instructions[i + target.len()..]
            .iter()
            .zip(i + target.len()..)
        {
            if let Instruction::Jnz(_, RegOrValue::Value(ref mut v)) = before
                && j.saturating_add_signed(isize::try_from(*v).unwrap()) < i
            {
                *v -= i64::try_from(replacement.len()).unwrap()
                    - i64::try_from(target.len()).unwrap();
            }
            result.push(before);
        }
        return result;
    }
    panic!("Target not found!");
}

#[allow(unused)]
fn print_program(instructions: &[Instruction]) {
    let mut targets = vec![false; instructions.len()];
    for (i, instr) in instructions.iter().enumerate() {
        if let &Instruction::Jnz(_, RegOrValue::Value(v)) = instr
            && let Some(j) = i.checked_add_signed(isize::try_from(v).unwrap())
            && (0..instructions.len()).contains(&j)
        {
            targets[j] = true;
        }
    }
    for (i, (instr, is_target)) in instructions.iter().zip(targets).enumerate() {
        if is_target {
            println!("{i:3}) > {instr}");
        } else {
            println!("{i:3})   {instr}");
        }
    }
}

fn extract_register_mapping(
    instructions: &[Instruction],
    target: &[Instruction],
) -> Option<RegisterMapping> {
    let mut mapping = RegisterMapping::new();
    for (&ins1, &ins2) in instructions.iter().zip(target) {
        match (ins1, ins2) {
            (Instruction::BinOp(op1, reg1, _), Instruction::BinOp(op2, reg2, _)) if op1 == op2 => {
                if !mapping.try_insert(reg1, reg2) {
                    return None;
                }
            }
            (
                Instruction::Jnz(RegOrValue::Reg(reg1), _),
                Instruction::Jnz(RegOrValue::Reg(reg2), _),
            ) => {
                if !mapping.try_insert(reg1, reg2) {
                    return None;
                }
            }
            (
                Instruction::Jnz(RegOrValue::Value(_), _),
                Instruction::Jnz(RegOrValue::Value(_), _),
            ) => {}
            _ => {
                return None;
            }
        }
        match (ins1, ins2) {
            (
                Instruction::BinOp(op1, _, RegOrValue::Reg(reg1)),
                Instruction::BinOp(op2, _, RegOrValue::Reg(reg2)),
            ) if op1 == op2 => {
                if !mapping.try_insert(reg1, reg2) {
                    return None;
                }
            }
            (
                Instruction::BinOp(op1, _, RegOrValue::Value(_)),
                Instruction::BinOp(op2, _, RegOrValue::Value(_)),
            ) if op1 == op2 => {}
            (
                Instruction::Jnz(_, RegOrValue::Reg(reg1)),
                Instruction::Jnz(_, RegOrValue::Reg(reg2)),
            ) => {
                if !mapping.try_insert(reg1, reg2) {
                    return None;
                }
            }
            (
                Instruction::Jnz(_, RegOrValue::Value(_)),
                Instruction::Jnz(_, RegOrValue::Value(_)),
            ) => {}
            _ => {
                return None;
            }
        }
    }
    Some(mapping)
}

#[derive(Debug, Clone, Default)]
struct RegisterMapping {
    forward: [Option<Reg>; Reg::all().len()],
    reverse: [Option<Reg>; Reg::all().len()],
}

impl RegisterMapping {
    fn new() -> Self {
        Self::default()
    }
    fn try_insert(&mut self, reg1: Reg, reg2: Reg) -> bool {
        match (self.forward[reg1 as usize], self.reverse[reg2 as usize]) {
            (Some(f), Some(r)) if f == reg2 && r == reg1 => true,
            (None, None) => {
                self.forward[reg1 as usize] = Some(reg2);
                self.reverse[reg2 as usize] = Some(reg1);
                true
            }
            _ => false,
        }
    }
    const fn reverse_reg(&self, reg: Reg) -> Option<Reg> {
        self.reverse[reg as usize]
    }
    fn reverse_reg_or_value(&self, mut val: RegOrValue) -> Option<RegOrValue> {
        match val {
            RegOrValue::Value(_) => {}
            RegOrValue::Reg(ref mut reg) => {
                *reg = self.reverse_reg(*reg)?;
            }
        }
        Some(val)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Pending,
    Stopped,
}

#[derive(Debug, Clone)]
struct Machine<'a> {
    instructions: &'a [Instruction],
    state: State,
    ip: usize,
    registers: [i64; Reg::all().len()],
    mul_count: usize,
}

impl<'a> Machine<'a> {
    pub const fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            instructions,
            state: State::Pending,
            ip: 0,
            registers: [0; Reg::all().len()],
            mul_count: 0,
        }
    }

    fn get_value(&self, source: RegOrValue) -> i64 {
        match source {
            RegOrValue::Reg(reg) => self[reg],
            RegOrValue::Value(val) => val,
        }
    }

    fn step(&mut self) {
        if self.state != State::Pending {
            return;
        }
        let Some(&instr) = self.instructions.get(self.ip) else {
            self.state = State::Stopped;
            return;
        };
        match instr {
            Instruction::BinOp(op, reg, rhs) => {
                let rhs = self.get_value(rhs);
                op.apply(&mut self[reg], rhs);
                if op == BinOp::Mul {
                    self.mul_count += 1;
                }
            }
            Instruction::Jnz(check, delta) => {
                if self.get_value(check) != 0 {
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
