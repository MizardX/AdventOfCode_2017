use std::ops::Add;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Invalid direction")]
    InvalidDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "n" => Self::North,
            "ne" => Self::NorthEast,
            "se" => Self::SouthEast,
            "s" => Self::South,
            "sw" => Self::SouthWest,
            "nw" => Self::NorthWest,
            _ => return Err(ParseError::InvalidDirection),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Axial {
    q: i64,
    r: i64,
}

impl Axial {
    const fn distance(self) -> u64 {
        (self.r.unsigned_abs()
            + self.q.unsigned_abs()
            + (self.r + self.q).unsigned_abs())
            / 2
    }
}

impl Add for Axial {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.r += rhs.r;
        self.q += rhs.q;
        self
    }
}

impl From<Direction> for Axial {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Self {
                q: 0,
                r: -1,
            },
            Direction::NorthEast => Self {
                q: 1,
                r: -1,
            },
            Direction::SouthEast => Self {
                q: 1,
                r: 0,
            },
            Direction::South => Self {
                q: 0,
                r: 1,
            },
            Direction::SouthWest => Self {
                q: -1,
                r: 1,
            },
            Direction::NorthWest => Self {
                q: -1,
                r: 0,
            },
        }
    }
}

impl Add<Direction> for Axial {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        let rhs_axial: Self = rhs.into();
        self + rhs_axial
    }
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Result<Vec<Direction>, ParseError> {
    input.split(',').map(str::parse).collect()
}

#[aoc(day11, part1)]
fn part_1(directions: &[Direction]) -> u64 {
    directions
        .iter()
        .copied()
        .fold(Axial::default(), Axial::add)
        .distance()
}

#[aoc(day11, part2)]
fn part_2(directions: &[Direction]) -> u64 {
    directions
        .iter()
        .copied()
        .scan(Axial::default(), |pos, dir| {
            *pos = *pos + dir;
            Some(pos.distance())
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("ne,ne,ne" => vec![Direction::NorthEast; 3])]
    #[test_case("ne,ne,sw,sw" => vec![Direction::NorthEast, Direction::NorthEast, Direction::SouthWest, Direction::SouthWest])]
    #[test_case("ne,ne,s,s" => vec![Direction::NorthEast, Direction::NorthEast, Direction::South, Direction::South])]
    #[test_case("se,sw,se,sw,sw" => vec![Direction::SouthEast, Direction::SouthWest, Direction::SouthEast, Direction::SouthWest, Direction::SouthWest])]
    fn test_parse(input: &str) -> Vec<Direction> {
        parse(input).unwrap()
    }

    #[test_case("ne,ne,ne" => 3)]
    #[test_case("ne,ne,sw,sw" => 0)]
    #[test_case("ne,ne,s,s" => 2)]
    #[test_case("se,sw,se,sw,sw" => 3)]
    fn test_part_1(input: &str) -> u64 {
        let directions = parse(input).unwrap();
        part_1(&directions)
    }
}
