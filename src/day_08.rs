use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error("Invalid operation")]
    InvalidOperation,
    #[error("Invalid comparison")]
    InvalidComparison,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Inc,
    Dec,
}

impl Operation {
    const fn apply(self, curr: &mut i64, delta: i64) -> i64 {
        match self {
            Self::Inc => *curr += delta,
            Self::Dec => *curr -= delta,
        }
        *curr
    }
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "inc" => Self::Inc,
            "dec" => Self::Dec,
            _ => return Err(ParseError::InvalidOperation),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Comparison {
    Less,
    LessEqual,
    Equal,
    GreaterEqual,
    Greater,
    NotEqual,
}

impl Comparison {
    fn eval(self, lhs: i64, rhs: i64) -> bool {
        matches!(
            (self, lhs.cmp(&rhs)),
            (Self::Less, Ordering::Less)
                | (Self::LessEqual, Ordering::Less | Ordering::Equal)
                | (Self::Equal, Ordering::Equal)
                | (Self::GreaterEqual, Ordering::Equal | Ordering::Greater)
                | (Self::Greater, Ordering::Greater)
                | (Self::NotEqual, Ordering::Less | Ordering::Greater)
        )
    }
}

impl FromStr for Comparison {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "<" => Self::Less,
            "<=" => Self::LessEqual,
            "==" => Self::Equal,
            ">=" => Self::GreaterEqual,
            ">" => Self::Greater,
            "!=" => Self::NotEqual,
            _ => return Err(ParseError::InvalidComparison),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    target: usize,
    operation: Operation,
    amount: i64,
    check: usize,
    comparison: Comparison,
    constant: i64,
}

impl Instruction {
    fn parse<'a>(
        line: &'a str,
        names: &mut Vec<String>,
        name_lookup: &mut HashMap<&'a str, usize>,
    ) -> Result<Self, ParseError> {
        let mut words = line.split(' ');
        let target = words.next().ok_or(ParseError::SyntaxError)?;
        let next_index = name_lookup.len();
        let target = match name_lookup.entry(target) {
            Entry::Occupied(occ) => *occ.get(),
            Entry::Vacant(vac) => {
                names.push(target.to_string());
                *vac.insert(next_index)
            }
        };
        let operation = words.next().ok_or(ParseError::SyntaxError)?;
        let operation = operation.parse()?;
        let amount: &str = words.next().ok_or(ParseError::SyntaxError)?;
        let amount = amount.parse()?;
        let if_ = words.next().ok_or(ParseError::SyntaxError)?;
        if if_ != "if" {
            return Err(ParseError::SyntaxError);
        }
        let check = words.next().ok_or(ParseError::SyntaxError)?;
        let next_index = name_lookup.len();
        let check = match name_lookup.entry(check) {
            Entry::Occupied(occ) => *occ.get(),
            Entry::Vacant(vac) => {
                names.push(check.to_string());
                *vac.insert(next_index)
            }
        };
        let comparison = words.next().ok_or(ParseError::SyntaxError)?;
        let comparison = comparison.parse()?;
        let constant = words.next().ok_or(ParseError::SyntaxError)?;
        let constant = constant.parse()?;
        if words.next().is_some() {
            return Err(ParseError::SyntaxError);
        }
        Ok(Self {
            target,
            operation,
            amount,
            check,
            comparison,
            constant,
        })
    }
}

struct Program {
    instructions: Vec<Instruction>,
    names: Vec<String>,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Result<Program, ParseError> {
    let mut string_pool = HashMap::new();
    let mut names = Vec::new();
    let instructions = input
        .lines()
        .map(|line| Instruction::parse(line, &mut names, &mut string_pool))
        .collect::<Result<_, _>>()?;
    Ok(Program {
        instructions,
        names,
    })
}

struct Machine<'a> {
    program: &'a Program,
    registers: Vec<i64>,
    max_value: i64,
}

impl<'a> Machine<'a> {
    fn new(program: &'a Program) -> Self {
        Self {
            program,
            registers: vec![0; program.names.len()],
            max_value: 0,
        }
    }

    fn run(&mut self) {
        for &instr in &self.program.instructions {
            let Instruction {
                target,
                operation,
                amount,
                check,
                comparison,
                constant,
            } = instr;
            let check_val = self.registers[check];
            if comparison.eval(check_val, constant) {
                let val = operation.apply(&mut self.registers[target], amount);
                self.max_value = self.max_value.max(val);
            }
        }
    }
}

#[aoc(day8, part1)]
fn part_1(program: &Program) -> i64 {
    let mut machine = Machine::new(program);
    machine.run();
    machine.registers.into_iter().max().unwrap()
}

#[aoc(day8, part2)]
fn part_2(program: &Program) -> i64 {
    let mut machine = Machine::new(program);
    machine.run();
    machine.max_value
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        b inc 5 if a > 1\n\
        a inc 1 if b < 5\n\
        c dec -10 if a >= 1\n\
        c inc -20 if c == 10\n\
    "
    .trim_ascii();

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(result.names, ["b", "a", "c"]);
        assert_eq!(
            result.instructions,
            [
                Instruction {
                    target: 0,
                    operation: Operation::Inc,
                    amount: 5,
                    check: 1,
                    comparison: Comparison::Greater,
                    constant: 1
                },
                Instruction {
                    target: 1,
                    operation: Operation::Inc,
                    amount: 1,
                    check: 0,
                    comparison: Comparison::Less,
                    constant: 5
                },
                Instruction {
                    target: 2,
                    operation: Operation::Dec,
                    amount: -10,
                    check: 1,
                    comparison: Comparison::GreaterEqual,
                    constant: 1
                },
                Instruction {
                    target: 2,
                    operation: Operation::Inc,
                    amount: -20,
                    check: 2,
                    comparison: Comparison::Equal,
                    constant: 10
                },
            ]
        );
    }

    #[test]
    fn test_instructions() {
        fn run(operation: Operation, comparison: Comparison, constant: i64) -> i64 {
            let names = vec!["a".to_string()];
            let instructions = vec![Instruction {
                target: 0,
                operation,
                amount: 1,
                check: 0,
                comparison,
                constant,
            }];
            let program = Program {
                instructions,
                names,
            };
            let mut machine = Machine::new(&program);
            machine.run();
            machine.registers[0]
        }
        assert_eq!(run(Operation::Inc, Comparison::Equal, 0), 1, "inc");
        assert_eq!(run(Operation::Dec, Comparison::Equal, 0), -1, "dec");

        assert_eq!(run(Operation::Inc, Comparison::Equal, -1), 0, "==-1");
        assert_eq!(run(Operation::Inc, Comparison::Equal, 0), 1, "==0");
        assert_eq!(run(Operation::Inc, Comparison::Equal, 1), 0, "==1");
        assert_eq!(run(Operation::Inc, Comparison::Less, -1), 0, "<-1");
        assert_eq!(run(Operation::Inc, Comparison::Less, 0), 0, "<0");
        assert_eq!(run(Operation::Inc, Comparison::Less, 1), 1, "<1");
        assert_eq!(run(Operation::Inc, Comparison::LessEqual, -1), 0, "<=-1");
        assert_eq!(run(Operation::Inc, Comparison::LessEqual, 0), 1, "<=0");
        assert_eq!(run(Operation::Inc, Comparison::LessEqual, 1), 1, "<=1");
        assert_eq!(run(Operation::Inc, Comparison::GreaterEqual, -1), 1, ">=-1");
        assert_eq!(run(Operation::Inc, Comparison::GreaterEqual, 0), 1, ">=0");
        assert_eq!(run(Operation::Inc, Comparison::GreaterEqual, 1), 0, ">=1");
        assert_eq!(run(Operation::Inc, Comparison::Greater, -1), 1, ">-1");
        assert_eq!(run(Operation::Inc, Comparison::Greater, 0), 0, ">0");
        assert_eq!(run(Operation::Inc, Comparison::Greater, 1), 0, ">1");
        assert_eq!(run(Operation::Inc, Comparison::NotEqual, -1), 1, "!=-1");
        assert_eq!(run(Operation::Inc, Comparison::NotEqual, 0), 0, "!=0");
        assert_eq!(run(Operation::Inc, Comparison::NotEqual, 1), 1, "!=1");
    }

    #[test]
    fn test_part_1_and_2() {
        let program = parse(EXAMPLE).unwrap();
        let mut machine = Machine::new(&program);
        machine.run();
        assert_eq!(machine.registers, [0, 1, -10]);
        assert_eq!(machine.max_value, 10);
    }

}
