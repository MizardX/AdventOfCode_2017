use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Component(u32, u32);

impl Component {
    const fn get_other(self, connector: u32) -> Option<u32> {
        if connector == self.0 {
            Some(self.1)
        } else if connector == self.1 {
            Some(self.0)
        } else {
            None
        }
    }

    const fn strength(self) -> u32 {
        self.0 + self.1
    }
}

impl FromStr for Component {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('/').ok_or(ParseError::SyntaxError)?;
        Ok(Self(a.parse()?, b.parse()?))
    }
}

impl std::fmt::Debug for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Self(a, b) = self;
        write!(f, "{a}/{b}")
    }
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Result<Vec<Component>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day24, part1)]
fn part_1(components: &[Component]) -> u32 {
    fn build_bridge(
        components: &mut [Component],
        index: usize,
        connector: u32,
        accum_strength: u32,
    ) -> u32 {
        if index >= components.len() {
            return accum_strength;
        }
        let mut best = accum_strength;
        for i in index..components.len() {
            if let Some(next) = components[i].get_other(connector) {
                components.swap(index, i);
                let candidate_strength = components[index].strength();
                let total_strength = build_bridge(
                    components,
                    index + 1,
                    next,
                    accum_strength + candidate_strength,
                );
                best = best.max(total_strength);
                components.swap(index, i);
            }
        }
        best
    }
    let mut components = components.to_vec();
    build_bridge(&mut components, 0, 0, 0)
}

#[aoc(day24, part2)]
fn part_2(components: &[Component]) -> u32 {
    fn build_bridge(
        components: &mut [Component],
        index: usize,
        connector: u32,
        accum_strength: u32,
    ) -> (usize, u32) {
        if index >= components.len() {
            return (index, accum_strength);
        }
        let mut best = (index, accum_strength);
        for i in index..components.len() {
            if let Some(next) = components[i].get_other(connector) {
                components.swap(index, i);
                let candidate_strength = components[index].strength();
                let total_strength = build_bridge(
                    components,
                    index + 1,
                    next,
                    accum_strength + candidate_strength,
                );
                best = best.max(total_strength);
                components.swap(index, i);
            }
        }
        best
    }
    let mut components = components.to_vec();
    build_bridge(&mut components, 0, 0, 0).1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0/2\n2/2\n2/3\n3/4\n3/5\n0/1\n10/1\n9/10";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(
            result,
            [
                Component(0, 2),
                Component(2, 2),
                Component(2, 3),
                Component(3, 4),
                Component(3, 5),
                Component(0, 1),
                Component(10, 1),
                Component(9, 10)
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let components = parse(EXAMPLE).unwrap();
        let result = part_1(&components);
        assert_eq!(result, 31);
    }

    #[test]
    fn test_part_2() {
        let components = parse(EXAMPLE).unwrap();
        let result = part_2(&components);
        assert_eq!(result, 19);
    }
}
