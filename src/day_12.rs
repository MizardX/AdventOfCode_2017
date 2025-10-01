use std::num::ParseIntError;

use thiserror::Error;

use crate::utils::UnionFind;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Result<Vec<(u16, u16)>, ParseError> {
    let mut result = Vec::new();
    for line in input.lines() {
        let (left, rights) = line.split_once(" <-> ").ok_or(ParseError::SyntaxError)?;
        let left = left.parse()?;
        for right in rights.split(", ") {
            let right = right.parse()?;
            result.push((left, right));
        }
    }
    Ok(result)
}

#[aoc(day12, part1)]
fn part_1(input: &[(u16, u16)]) -> usize {
    let num_programs = input.iter().map(|&(a, b)| a.max(b)).max().unwrap() as usize + 1;
    let mut uf = UnionFind::new(num_programs);
    for &(a, b) in input {
        uf.union(a as usize, b as usize);
    }
    let target_group = uf.find(0);
    (0..num_programs)
        .filter(|&p| uf.find(p) == target_group)
        .count()
}

#[aoc(day12, part2)]
fn part_2(input: &[(u16, u16)]) -> usize {
    let num_programs = input.iter().map(|&(a, b)| a.max(b)).max().unwrap() as usize + 1;
    let mut uf = UnionFind::new(num_programs);
    for &(a, b) in input {
        uf.union(a as usize, b as usize);
    }
    uf.num_groups()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        0 <-> 2\n\
        1 <-> 1\n\
        2 <-> 0, 3, 4\n\
        3 <-> 2, 4\n\
        4 <-> 2, 3, 6\n\
        5 <-> 6\n\
        6 <-> 4, 5\n\
    "
    .trim_ascii();

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();

        assert_eq!(
            result,
            [
                (0, 2),
                (1, 1),
                (2, 0),
                (2, 3),
                (2, 4),
                (3, 2),
                (3, 4),
                (4, 2),
                (4, 3),
                (4, 6),
                (5, 6),
                (6, 4),
                (6, 5),
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE).unwrap();
        let result = part_1(&input);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE).unwrap();
        let result = part_2(&input);
        assert_eq!(result, 2);
    }
}
