use std::{collections::VecDeque, num::ParseIntError, ops::Index, str::FromStr};

use thiserror::Error;

#[derive(Debug, Error, Clone)]
enum ParseError {
    #[error("Unknown state")]
    UnknownState,
    #[error("Unknown symbol")]
    UnknownSymbol,
    #[error("Invalid direction")]
    InvalidDirection,
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}


fn parse_line<T>(line: Option<&str>, prefix: &str, suffix: &str) -> Result<T, ParseError>
where
    T: FromStr,
    ParseError: From<T::Err>,
{
    Ok(line
        .ok_or(ParseError::SyntaxError)?
        .strip_prefix(prefix)
        .ok_or(ParseError::SyntaxError)?
        .strip_suffix(suffix)
        .ok_or(ParseError::SyntaxError)?
        .parse()?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum StateId {
    #[default]
    A,
    B,
    C,
    D,
    E,
    F,
}

impl StateId {
    const fn all() -> [Self; 6] {
        [Self::A, Self::B, Self::C, Self::D, Self::E, Self::F]
    }
}

impl FromStr for StateId {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            "E" => Self::E,
            "F" => Self::F,
            _ => return Err(ParseError::UnknownState),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Symbol {
    #[default]
    Zero,
    One,
}

impl Symbol {
    const fn all() -> [Self; 2] {
        [Self::Zero, Self::One]
    }
}

impl FromStr for Symbol {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0" => Self::Zero,
            "1" => Self::One,
            _ => return Err(ParseError::UnknownSymbol),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Direction {
    #[default]
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "left" => Self::Left,
            "right" => Self::Right,
            _ => return Err(ParseError::InvalidDirection),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Action {
    write: Symbol,
    move_direction: Direction,
    next_state: StateId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {
    id: StateId,
    transitions: [Action; Symbol::all().len()],
}

impl FromStr for State {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id = parse_line(lines.next(), "In state ", ":")?;
        let mut transitions = [Action::default(); Symbol::all().len()];
        while let Some(line) = lines.next() {
            let symbol: Symbol = parse_line(Some(line), "  If the current value is ", ":")?;
            transitions[symbol as usize] = Action {
                write: parse_line(lines.next(), "    - Write the value ", ".")?,
                move_direction: parse_line(lines.next(), "    - Move one slot to the ", ".")?,
                next_state: parse_line(lines.next(), "    - Continue with state ", ".")?,
            };
        }
        Ok(Self { id, transitions })
    }
}

impl Index<Symbol> for State {
    type Output = Action;

    fn index(&self, index: Symbol) -> &Self::Output {
        &self.transitions[index as usize]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Blueprint {
    initial_state: StateId,
    checksum_after: usize,
    states: [State; StateId::all().len()],
}

impl Index<StateId> for Blueprint {
    type Output = State;

    fn index(&self, index: StateId) -> &Self::Output {
        &self.states[index as usize]
    }
}

impl FromStr for Blueprint {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks = s.split("\n\n");
        let mut lines = chunks.next().ok_or(ParseError::SyntaxError)?.lines();
        let initial_state: StateId = parse_line(lines.next(), "Begin in state ", ".")?;
        let checksum_after: usize = parse_line(
            lines.next(),
            "Perform a diagnostic checksum after ",
            " steps.",
        )?;
        let mut states = [State::default(); StateId::all().len()];
        for (ix, chunk) in chunks.enumerate() {
            states[ix] = chunk.parse()?;
        }
        Ok(Self {
            initial_state,
            checksum_after,
            states,
        })
    }
}

#[aoc_generator(day25)]
fn parse(input: &str) -> Result<Blueprint, ParseError> {
    input.parse()
}

#[aoc(day25, part1)]
fn part_1(blueprint: &Blueprint) -> usize {
    let mut machine = Machine::new(blueprint);
    machine.run();
    machine.checksum(Symbol::One)
}

struct Machine<'a> {
    blueprint: &'a Blueprint,
    state: StateId,
    position: usize,
    tape: VecDeque<Symbol>,
}

impl<'a> Machine<'a> {
    fn new(blueprint: &'a Blueprint) -> Self {
        Self {
            blueprint,
            state: blueprint.initial_state,
            position: 0,
            tape: [Symbol::default()].into(), // so tape[position] has a value
        }
    }

    fn move_left(&mut self) {
        if self.position == 0 {
            // push_front moves the zero position.
            self.tape.push_front(Symbol::default());
        } else {
            self.position -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.position == self.tape.len() - 1 {
            self.tape.push_back(Symbol::default());
        }
        self.position += 1;
    }

    fn read(&self) -> Symbol {
        self.tape[self.position]
    }

    fn write(&mut self, symbol: Symbol) {
        self.tape[self.position] = symbol;
    }

    fn tick(&mut self) {
        let &Action {
            write,
            move_direction,
            next_state,
        } = &self.blueprint[self.state][self.read()];
        self.write(write);
        match move_direction {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
        self.state = next_state;
    }

    fn run(&mut self) {
        for _ in 0..self.blueprint.checksum_after {
            self.tick();
        }
    }

    fn checksum(&self, symbol: Symbol) -> usize {
        self.tape.iter().filter(|&&s| s == symbol).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(result.initial_state, StateId::A);
        assert_eq!(result.checksum_after, 6);
        assert_eq!(
            result.states[StateId::A as usize],
            State {
                id: StateId::A,
                transitions: [
                    Action {
                        write: Symbol::One,
                        move_direction: Direction::Right,
                        next_state: StateId::B,
                    },
                    Action {
                        write: Symbol::Zero,
                        move_direction: Direction::Left,
                        next_state: StateId::B,
                    }
                ]
            }
        );
        assert_eq!(
            result.states[StateId::B as usize],
            State {
                id: StateId::B,
                transitions: [
                    Action {
                        write: Symbol::One,
                        move_direction: Direction::Left,
                        next_state: StateId::A,
                    },
                    Action {
                        write: Symbol::One,
                        move_direction: Direction::Right,
                        next_state: StateId::A,
                    }
                ]
            }
        );
    }

    #[test]
    fn test_part_1() {
        let blueprint = parse(EXAMPLE).unwrap();
        let result = part_1(&blueprint);
        assert_eq!(result, 3);
    }
}
