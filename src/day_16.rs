use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
    #[error("Invalid name")]
    InvalidName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Name {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
}

impl Name {
    pub const fn all() -> [Self; 16] {
        [
            Self::A,
            Self::B,
            Self::C,
            Self::D,
            Self::E,
            Self::F,
            Self::G,
            Self::H,
            Self::I,
            Self::J,
            Self::K,
            Self::L,
            Self::M,
            Self::N,
            Self::O,
            Self::P,
        ]
    }
}

impl FromStr for Name {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "a" => Self::A,
            "b" => Self::B,
            "c" => Self::C,
            "d" => Self::D,
            "e" => Self::E,
            "f" => Self::F,
            "g" => Self::G,
            "h" => Self::H,
            "i" => Self::I,
            "j" => Self::J,
            "k" => Self::K,
            "l" => Self::L,
            "m" => Self::M,
            "n" => Self::N,
            "o" => Self::O,
            "p" => Self::P,
            _ => return Err(ParseError::InvalidName),
        })
    }
}

impl From<Name> for usize {
    fn from(value: Name) -> Self {
        value as Self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    /// Spin sX
    Spin(usize),
    /// Exchange xA/B
    Exchange(usize, usize),
    /// Partner pA/B
    Partner(Name, Name),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(rest) = s.strip_prefix("s") {
            Self::Spin(rest.parse()?)
        } else if let Some(rest) = s.strip_prefix("x") {
            let (a, b) = rest.split_once('/').ok_or(ParseError::SyntaxError)?;
            Self::Exchange(a.parse()?, b.parse()?)
        } else if let Some(rest) = s.strip_prefix("p") {
            let (a, b) = rest.split_once('/').ok_or(ParseError::SyntaxError)?;
            Self::Partner(a.parse()?, b.parse()?)
        } else {
            return Err(ParseError::SyntaxError);
        })
    }
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Result<Vec<Instruction>, ParseError> {
    input.split(',').map(str::parse).collect()
}

#[aoc(day16, part1)]
fn part_1(instructions: &[Instruction]) -> String {
    slow_dance::<16>(instructions)
}

fn slow_dance<const N: usize>(instructions: &[Instruction]) -> String {
    let mut programs: [Name; N] = Name::all()[..N].try_into().unwrap();
    for &instr in instructions {
        match instr {
            Instruction::Spin(k) => programs.rotate_right(k),
            Instruction::Exchange(a, b) => programs.swap(a, b),
            Instruction::Partner(a, b) => {
                let a = programs.iter().position(|&p| p == a).unwrap();
                let b = programs.iter().position(|&p| p == b).unwrap();
                programs.swap(a, b);
            }
        }
    }
    unsafe { String::from_utf8_unchecked(programs.map(|p| p as u8 + b'a').to_vec()) }
}

#[aoc(day16, part2)]
fn part_2(instructions: &[Instruction]) -> String {
    const TIMES: u32 = 1_000_000_000;
    fast_dance::<16>(instructions, TIMES)
}

fn fast_dance<const N: usize>(instructions: &[Instruction], times: u32) -> String {
    let all_names: [Name; N] = Name::all()[..N].try_into().unwrap();

    let mut program = all_names;
    for &instr in instructions {
        match instr {
            Instruction::Spin(k) => program.rotate_right(k),
            Instruction::Exchange(a, b) => program.swap(a, b),
            Instruction::Partner(..) => {}
        }
    }
    let position_permutation = all_names.map(|n| program.iter().position(|&x| x == n).unwrap());
    let position_permutation = power_permutation(position_permutation, times);

    let mut program = all_names;
    for &instr in instructions {
        if let Instruction::Partner(a, b) = instr {
            let a = program.iter().position(|&p| p == a).unwrap();
            let b = program.iter().position(|&p| p == b).unwrap();
            program.swap(a, b);
        }
    }
    let value_permutation = power_permutation(program, times);

    let result = position_permutation.map(|x| value_permutation[x]);

    unsafe { String::from_utf8_unchecked(result.map(|p| p as u8 + b'a').to_vec()) }
}

fn power_permutation<T: Copy + Into<usize>, const N: usize>(
    mut permutation: [T; N],
    mut power: u32,
) -> [T; N] {
    if power == 0 {
        unimplemented!("Power 0 not supported")
    }
    while power & 1 == 0 {
        permutation = square_permutation(permutation);
        power /= 2;
    }
    let mut base = permutation;
    power -= 1;
    while power > 0 {
        if power & 1 == 0 {
            permutation = square_permutation(permutation);
            power /= 2;
        } else {
            base = multiply_permutations(base, permutation);
            power -= 1;
        }
    }
    base
}

fn square_permutation<T: Copy + Into<usize>, const N: usize>(permutation: [T; N]) -> [T; N] {
    multiply_permutations(permutation, permutation)
}

fn multiply_permutations<T: Copy + Into<usize>, const N: usize>(
    permutation1: [T; N],
    permutation2: [T; N],
) -> [T; N] {
    permutation1.map(|x| permutation2[x.into()])
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "s1,x3/4,pe/b";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(
            result,
            [
                Instruction::Spin(1),
                Instruction::Exchange(3, 4),
                Instruction::Partner(Name::E, Name::B),
            ]
        );
    }

    #[test]
    fn test_slow_dance() {
        let instructions = parse(EXAMPLE).unwrap();
        let result = slow_dance::<5>(&instructions);
        assert_eq!(result, "baedc");
    }

    #[test]
    fn test_fast_dance() {
        let instructions = parse(EXAMPLE).unwrap();
        let result = fast_dance::<5>(&instructions, 2);
        assert_eq!(result, "ceadb");
    }
}
