use thiserror::Error;

use crate::utils::{Grid, GridParseError};

#[derive(Debug, Error)]
enum ParseError {
    #[error("Invalid tile")]
    InvalidTile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Vertical,
    Horizontal,
    Corner,
    Letter(u8),
}

impl TryFrom<u8> for Tile {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b' ' => Self::Empty,
            b'-' => Self::Horizontal,
            b'|' => Self::Vertical,
            b'+' => Self::Corner,
            b'A'..=b'Z' => Self::Letter(value),
            _ => return Err(ParseError::InvalidTile),
        })
    }
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Result<Grid<Tile>, GridParseError<ParseError>> {
    input.parse()
}

#[aoc(day19, part1)]
fn part_1(map: &Grid<Tile>) -> String {
    let mut found_letters = Vec::new();
    walk_map(map, |tile| {
        if let Tile::Letter(ch) = tile {
            found_letters.push(ch);
        }
    });
    unsafe { String::from_utf8_unchecked(found_letters) }
}

#[aoc(day19, part2)]
fn part_2(map: &Grid<Tile>) -> usize {
    let mut steps = 0;
    walk_map(map, |_| steps += 1);
    steps
}

fn walk_map<F>(map: &Grid<Tile>, mut visit: F)
where
    F: FnMut(Tile),
{
    let start = (0..map.cols())
        .map(|c| (0, c))
        .find(|&pos| map[pos] == Tile::Vertical)
        .unwrap();
    let mut came_from = start;
    visit(map[came_from]);
    let mut current = (start.0 + 1, start.1);
    while map[current] != Tile::Empty {
        visit(map[current]);
        let next = match map[current] {
            Tile::Letter(..) | Tile::Vertical | Tile::Horizontal => {
                (current.0 * 2 - came_from.0, current.1 * 2 - came_from.1)
            }
            Tile::Empty => panic!("Entered the void!"),
            Tile::Corner => [
                current.0.checked_sub(1).map(|r| (r, current.1)),
                current.1.checked_sub(1).map(|c| (current.0, c)),
                (current.0 + 1 < map.rows()).then_some((current.0 + 1, current.1)),
                (current.1 + 1 < map.cols()).then_some((current.0, current.1 + 1)),
            ]
            .into_iter()
            .flatten()
            .find(|&pos| map[pos] != Tile::Empty && pos != came_from)
            .unwrap(),
        };
        (came_from, current) = (current, next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        .....|..........\n\
        .....|..+--+....\n\
        .....A..|..C....\n\
        .F---|--|-E---+.\n\
        .....|..|..|..D.\n\
        .....+B-+..+--+.\
    ";

    #[test]
    fn test_parse() {
        const X: Tile = Tile::Empty;
        const V: Tile = Tile::Vertical;
        const H: Tile = Tile::Horizontal;
        const P: Tile = Tile::Corner;
        const A: Tile = Tile::Letter(b'A');
        const B: Tile = Tile::Letter(b'B');
        const C: Tile = Tile::Letter(b'C');
        const D: Tile = Tile::Letter(b'D');
        const E: Tile = Tile::Letter(b'E');
        const F: Tile = Tile::Letter(b'F');
        let map = parse(&EXAMPLE.replace('.', " ")).unwrap();
        let expected = [
            [X, X, X, X, X, V, X, X, X, X, X, X, X, X, X, X],
            [X, X, X, X, X, V, X, X, P, H, H, P, X, X, X, X],
            [X, X, X, X, X, A, X, X, V, X, X, C, X, X, X, X],
            [X, F, H, H, H, V, H, H, V, H, E, H, H, H, P, X],
            [X, X, X, X, X, V, X, X, V, X, X, V, X, X, D, X],
            [X, X, X, X, X, P, B, H, P, X, X, P, H, H, P, X],
        ];
        assert_eq!(map.rows(), expected.len());
        assert_eq!(map.cols(), expected[0].len());
        for (r, row) in expected.into_iter().enumerate() {
            for (c, cell) in row.into_iter().enumerate() {
                assert_eq!(map[(r, c)], cell);
            }
        }
    }

    #[test]
    fn test_part_1() {
        let map = parse(&EXAMPLE.replace('.', " ")).unwrap();
        let result = part_1(&map);
        assert_eq!(result, "ABCDEF");
    }

    #[test]
    fn test_part_2() {
        let map = parse(&EXAMPLE.replace('.', " ")).unwrap();
        let result = part_2(&map);
        assert_eq!(result, 38);
    }
}
