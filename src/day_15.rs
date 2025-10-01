use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

struct Generator {
    last_value: u64,
    multiply: u64,
    filter: Option<u64>,
}

impl Generator {
    pub const fn new(last_value: u64, multiply: u64, filter: Option<u64>) -> Self {
        Self {
            last_value,
            multiply,
            filter,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.last_value = (self.last_value * self.multiply) % 2_147_483_647;
            if self
                .filter
                .is_none_or(|multiple_of| self.last_value.is_multiple_of(multiple_of))
            {
                return Some(self.last_value);
            }
        }
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Input {
    generator_a: u64,
    generator_b: u64,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let generator_a = lines
            .next()
            .ok_or(ParseError::SyntaxError)?
            .strip_prefix("Generator A starts with ")
            .ok_or(ParseError::SyntaxError)?
            .parse()?;
        let generator_b = lines
            .next()
            .ok_or(ParseError::SyntaxError)?
            .strip_prefix("Generator B starts with ")
            .ok_or(ParseError::SyntaxError)?
            .parse()?;
        if lines.next().is_some() {
            return Err(ParseError::SyntaxError);
        }
        Ok(Self {
            generator_a,
            generator_b,
        })
    }
}

#[aoc_generator(day15)]
fn parse(input: &str) -> Result<Input, ParseError> {
    input.parse()
}

#[aoc(day15, part1)]
fn part_1(input: &Input) -> usize {
    let generator_a = Generator::new(input.generator_a, 16_807, None);
    let generator_b = Generator::new(input.generator_b, 48_271, None);
    generator_a
        .zip(generator_b)
        .take(40_000_000)
        .filter(|(a, b)| (a ^ b).trailing_zeros() >= 16)
        .count()
}

#[aoc(day15, part2)]
fn part_2(input: &Input) -> usize {
    let generator_a = Generator::new(input.generator_a, 16_807, Some(4));
    let generator_b = Generator::new(input.generator_b, 48_271, Some(8));
    generator_a
        .zip(generator_b)
        .take(5_000_000)
        .filter(|(a, b)| (a ^ b).trailing_zeros() >= 16)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Input {
            generator_a: 65,
            generator_b: 8921,
        };
        let result = part_1(&input);
        assert_eq!(result, 588);
    }
    #[test]
    fn test_part_2() {
        let input = Input {
            generator_a: 65,
            generator_b: 8921,
        };
        let result = part_2(&input);
        assert_eq!(result, 309);
    }
}
