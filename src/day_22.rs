use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Cell invalid state")]
    InvalidState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum State {
    Clean = b'.',
    Weakened = b'W',
    Infected = b'#',
    Flagged = b'F',
}

impl TryFrom<u8> for State {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'.' => Self::Clean,
            b'#' => Self::Infected,
            _ => return Err(ParseError::InvalidState),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    cells: HashMap<(i32, i32), State>,
    fallback: State, // default for Index trait
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = HashMap::new();
        let height = i32::try_from(s.lines().count()).unwrap();
        let width = i32::try_from(s.lines().next().unwrap().len()).unwrap();
        let offset_r = -height / 2;
        let offset_c = -width / 2;
        for (line, r) in s.lines().zip(0..) {
            for (ch, c) in line.bytes().zip(0..) {
                cells.insert((r + offset_r, c + offset_c), ch.try_into()?);
            }
        }
        Ok(Self {
            cells,
            fallback: State::Clean,
        })
    }
}

impl Index<(i32, i32)> for Map {
    type Output = State;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        self.cells.get(&index).unwrap_or(&self.fallback)
    }
}

impl IndexMut<(i32, i32)> for Map {
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
        self.cells.entry(index).or_insert(self.fallback)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
    const fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }
    const fn turn_around(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
    const fn step(self, (mut r, mut c): (i32, i32)) -> (i32, i32) {
        match self {
            Self::Up => r -= 1,
            Self::Right => c += 1,
            Self::Down => r += 1,
            Self::Left => c -= 1,
        }
        (r, c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Infection {
    position: (i32, i32),
    direction: Direction,
    count_infected: usize,
}

impl Infection {
    fn new() -> Self {
        Self::default()
    }
    fn basic_step(&mut self, map: &mut Map) {
        match map[self.position] {
            State::Clean => {
                map[self.position] = State::Infected;
                self.direction = self.direction.turn_left();
                self.position = self.direction.step(self.position);
                self.count_infected += 1;
            }
            State::Infected => {
                map[self.position] = State::Clean;
                self.direction = self.direction.turn_right();
                self.position = self.direction.step(self.position);
            }
            State::Weakened | State::Flagged => unimplemented!(),
        }
    }
    fn evolved_step(&mut self, map: &mut Map) {
        match map[self.position] {
            State::Clean => {
                map[self.position] = State::Weakened;
                self.direction = self.direction.turn_left();
                self.position = self.direction.step(self.position);
            }
            State::Weakened => {
                map[self.position] = State::Infected;
                self.position = self.direction.step(self.position);
                self.count_infected += 1;
            }
            State::Infected => {
                map[self.position] = State::Flagged;
                self.direction = self.direction.turn_right();
                self.position = self.direction.step(self.position);
            }
            State::Flagged => {
                map[self.position] = State::Clean;
                self.direction = self.direction.turn_around();
                self.position = self.direction.step(self.position);
            }
        }
    }
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Result<Map, ParseError> {
    input.parse()
}

#[aoc(day22, part1)]
fn part_1(map: &Map) -> usize {
    let mut map = map.clone();
    let mut infection = Infection::new();
    for _ in 0..10_000 {
        infection.basic_step(&mut map);
    }
    infection.count_infected
}

#[aoc(day22, part2)]
fn part_2(map: &Map) -> usize {
    let mut map = map.clone();
    let mut infection = Infection::new();
    for _ in 0..10_000_000 {
        infection.evolved_step(&mut map);
    }
    infection.count_infected
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "..#\n#..\n...";

    #[test]
    fn test_parse() {
        const C: State = State::Clean;
        const I: State = State::Infected;
        let result = parse(EXAMPLE).unwrap();
        let expected = [[C, C, I], [I, C, C], [C, C, C]];
        for (row, r) in expected.into_iter().zip(-1..) {
            for (cell, c) in row.into_iter().zip(-1..) {
                assert_eq!(result[(r, c)], cell);
            }
        }
    }

    #[test]
    fn test_part_1() {
        let map = parse(EXAMPLE).unwrap();
        let result = part_1(&map);
        assert_eq!(result, 5_587);
    }

    #[test]
    fn test_part_2() {
        let map = parse(EXAMPLE).unwrap();
        let result = part_2(&map);
        assert_eq!(result, 2_511_944);
    }
}
