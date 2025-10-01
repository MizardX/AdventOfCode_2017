use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Layer {
    depth: u64,
    range: u64,
}

impl Layer {
    const fn severity(&self) -> u64 {
        if self.is_safe_delay(0) {
            0
        } else {
            self.depth * self.range
        }
    }

    const fn is_safe_delay(&self, delay: u64) -> bool {
        !(self.depth + delay).is_multiple_of(2 * self.range - 2)
    }
}

impl FromStr for Layer {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (depth, range) = s.split_once(": ").ok_or(ParseError::SyntaxError)?;
        Ok(Self {
            depth: depth.parse()?,
            range: range.parse()?,
        })
    }
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Result<Vec<Layer>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day13, part1)]
fn part_1(layers: &[Layer]) -> u64 {
    layers.iter().map(Layer::severity).sum()
}

#[aoc(day13, part2)]
fn part_2(layers: &[Layer]) -> u64 {
    (0..10_000_000)
        .find(|&delay| layers.iter().all(|l| l.is_safe_delay(delay)))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        0: 3\n\
        1: 2\n\
        4: 4\n\
        6: 4\n\
    "
    .trim_ascii();

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(
            result,
            [
                Layer { depth: 0, range: 3 },
                Layer { depth: 1, range: 2 },
                Layer { depth: 4, range: 4 },
                Layer { depth: 6, range: 4 }
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let layers = parse(EXAMPLE).unwrap();
        let result = part_1(&layers);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_part_2() {
        let layers = parse(EXAMPLE).unwrap();
        let result = part_2(&layers);
        assert_eq!(result, 10);
    }
}
