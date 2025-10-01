use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Clone, Copy)]
struct UnionFindNode {
    parent: usize,
    size: usize,
}

#[derive(Debug, Clone)]
struct UnionFind {
    nodes: Vec<UnionFindNode>,
    num_groups: usize,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let nodes = (0..size)
            .map(|parent| UnionFindNode { parent, size: 1 })
            .collect();
        Self {
            nodes,
            num_groups: size,
        }
    }

    fn find(&mut self, mut index: usize) -> usize {
        let mut parent = self.nodes[index].parent;
        while parent != index {
            let grand_parent = self.nodes[parent].parent;
            self.nodes[index].parent = grand_parent;
            index = grand_parent;
            parent = self.nodes[index].parent;
        }
        index
    }

    fn union(&mut self, mut index1: usize, mut index2: usize) -> bool {
        index1 = self.find(index1);
        index2 = self.find(index2);
        if index1 == index2 {
            return false;
        }
        if self.nodes[index1].size > self.nodes[index2].size {
            (index1, index2) = (index2, index1);
        }
        self.nodes[index2].parent = index1;
        self.nodes[index1].size += self.nodes[index2].size;
        self.num_groups -= 1;
        true
    }
}

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
    uf.num_groups
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
