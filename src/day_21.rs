use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SmallRule {
    pattern: u8,      // 4 bits
    replacement: u16, // 9 bits
}

impl SmallRule {
    fn variations(self) -> [Self; 8] {
        let mut res = [self; 8];
        let mut ix = 0;
        for transpose in [
            self.pattern,
            self.pattern & 0b10_01 | (self.pattern & 0b01_00) >> 1 | (self.pattern & 0b00_10) << 1,
        ] {
            for mirror_x in [
                transpose,
                (transpose & 0b10_10) >> 1 | (transpose & 0b01_01) << 1,
            ] {
                for mirror_y in [
                    mirror_x,
                    (mirror_x & 0b11_00) >> 2 | (mirror_x & 0b00_11) << 2,
                ] {
                    res[ix].pattern = mirror_y;
                    ix += 1;
                }
            }
        }
        res
    }
}

impl FromStr for SmallRule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s.split_once(" => ").ok_or(ParseError::SyntaxError)?;
        let pattern = lhs
            .as_bytes()
            .chunks(3)
            .map(|chunk| chunk[..2].iter().fold(0, |mask, &ch| (mask << 1) | ch & 1))
            .fold(0, |mask, row| (mask << 2) | row);
        let replacement = rhs
            .as_bytes()
            .chunks(4)
            .map(|chunk| {
                chunk[..3]
                    .iter()
                    .fold(0, |mask, &ch| (mask << 1) | u16::from(ch & 1))
            })
            .fold(0, |mask, row| (mask << 3) | row);
        Ok(Self {
            pattern,
            replacement,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LargeRule {
    pattern: u16,     // 9 bits
    replacement: u16, // 16 bits
}

impl LargeRule {
    fn variations(self) -> [Self; 8] {
        let mut res = [self; 8];
        let mut ix = 0;
        let pattern = self.pattern;
        for transpose in [
            pattern,
            (pattern & 0b001_000_000) >> 4
                | (pattern & 0b010_001_000) >> 2
                | (pattern & 0b100_010_001)
                | (pattern & 0b000_100_010) << 2
                | (pattern & 0b000_000_100) << 4,
        ] {
            for mirror_x in [
                transpose,
                (transpose & 0b100_100_100) >> 2
                    | transpose & 0b010_010_010
                    | (transpose & 0b001_001_001) << 2,
            ] {
                for mirror_y in [
                    mirror_x,
                    (mirror_x & 0b111_000_000) >> 6
                        | mirror_x & 0b000_111_000
                        | (mirror_x & 0b000_000_111) << 6,
                ] {
                    res[ix].pattern = mirror_y;
                    ix += 1;
                }
            }
        }
        res
    }
}

impl FromStr for LargeRule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s.split_once(" => ").ok_or(ParseError::SyntaxError)?;
        let pattern = lhs
            .as_bytes()
            .chunks(4)
            .map(|chunk| {
                chunk[..3]
                    .iter()
                    .fold(0, |mask, &ch| (mask << 1) | u16::from(ch & 1))
            })
            .fold(0, |mask, row| (mask << 3) | row);
        let replacement = rhs
            .as_bytes()
            .chunks(5)
            .map(|chunk| {
                chunk[..4]
                    .iter()
                    .fold(0, |mask, &ch| (mask << 1) | u16::from(ch & 1))
            })
            .fold(0, |mask, row| (mask << 4) | row);
        Ok(Self {
            pattern,
            replacement,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rule {
    Small(SmallRule),
    Large(LargeRule),
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.len() {
            20 => Self::Small(s.parse()?),
            34 => Self::Large(s.parse()?),
            _ => return Err(ParseError::SyntaxError),
        })
    }
}

#[aoc_generator(day21)]
fn parse(input: &str) -> Result<Vec<Rule>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day21, part1)]
fn part_1(rules: &[Rule]) -> u32 {
    let (small, large) = create_lookups(rules);
    let mut pattern: Vec<u64> = vec![0b010, 0b001, 0b111];
    let mut next = Vec::new();
    for _ in 1..=5 {
        expand_grid(&pattern, &mut next, &small, &large);
        (pattern, next) = (next, pattern);
    }
    pattern.into_iter().map(u64::count_ones).sum()
}

fn create_lookups(rules: &[Rule]) -> ([u16; 16], [u16; 512]) {
    let mut small = [0; 16];
    let mut large = [0; 512];
    for pat in rules {
        match pat {
            Rule::Small(small_rule) => {
                for var in small_rule.variations() {
                    small[var.pattern as usize] = var.replacement;
                }
            }
            Rule::Large(large_rule) => {
                for var in large_rule.variations() {
                    large[var.pattern as usize] = var.replacement;
                }
            }
        }
    }
    (small, large)
}

fn expand_grid(pattern: &[u64], next: &mut Vec<u64>, small: &[u16; 16], large: &[u16; 512]) {
    next.clear();
    let n = pattern.len();
    #[expect(clippy::cast_possible_truncation)]
    #[expect(clippy::identity_op)]
    if n & 1 == 0 {
        next.reserve(n / 2 * 3);
        for (&r1, &r2) in pattern.iter().zip(&pattern[1..]).step_by(2) {
            let (mut n1, mut n2, mut n3) = (0, 0, 0);
            let mut shift = 0;
            for region in 0..n / 2 {
                let key = ((r1 >> (2 * region)) & 0b11) << 2 | (r2 >> (2 * region)) & 0b11;
                let img = u64::from(small[key as usize]);
                n1 |= (img & 0b111_000_000) >> 6 << shift;
                n2 |= (img & 0b000_111_000) >> 3 << shift;
                n3 |= (img & 0b000_000_111) >> 0 << shift;
                shift += 3;
            }
            next.push(n1);
            next.push(n2);
            next.push(n3);
        }
    } else {
        next.reserve(n / 3 * 4);

        for ((&r1, &r2), &r3) in pattern
            .iter()
            .zip(&pattern[1..])
            .zip(&pattern[2..])
            .step_by(3)
        {
            let (mut n1, mut n2, mut n3, mut n4) = (0, 0, 0, 0);
            let mut shift = 0;
            for region in 0..n / 3 {
                let key = ((r1 >> (3 * region)) & 0b111) << 6
                    | ((r2 >> (3 * region)) & 0b111) << 3
                    | (r3 >> (3 * region)) & 0b111;
                let img = u64::from(large[key as usize]);
                n1 |= (img & 0b1111_0000_0000_0000) >> 12 << shift;
                n2 |= (img & 0b0000_1111_0000_0000) >> 8 << shift;
                n3 |= (img & 0b0000_0000_1111_0000) >> 4 << shift;
                n4 |= (img & 0b0000_0000_0000_1111) >> 0 << shift;
                shift += 4;
            }
            next.push(n1);
            next.push(n2);
            next.push(n3);
            next.push(n4);
        }
    }
}

#[aoc(day21, part2)]
fn part_2(rules: &[Rule]) -> usize {
    let (small, large) = create_lookups(rules);
    let mut lookup = HashMap::new();
    let mut pending: VecDeque<_> = [0b010_001_111].into();
    let mut values = Vec::new();
    while let Some(pat) = pending.pop_front() {
        if lookup.contains_key(&pat) {
            continue;
        }
        let next = count_expanded(pat, &small, &large);
        pending.extend(next.iter().map(|&(n, _)| n));
        lookup.insert(pat, (next, values.len()));
        values.push(pat);
    }
    let n = values.len();
    let mut matrix = vec![vec![0; n]; n];
    for &(ref nexts, src_index) in lookup.values() {
        for &(dst, count) in nexts {
            let dst_index = lookup[&dst].1;
            matrix[src_index][dst_index] = count;
        }
    }
    let mut counts = vec![0; n];
    let mut next = vec![0; n];
    let start_index = lookup[&0b010_001_111].1;
    counts[start_index] = 1;
    for _ in 0..18 / 3 {
        next.fill(0);
        for (i, &cnt) in counts.iter().enumerate() {
            for (j, mult) in matrix[i].iter().enumerate() {
                next[j] += cnt * mult;
            }
        }
        (counts, next) = (next, counts);
    }
    values
        .iter()
        .zip(&counts)
        .map(|(&pat, &cnt)| usize::try_from(pat.count_ones()).unwrap() * cnt)
        .sum()
}

fn count_expanded(pattern: u16, small: &[u16; 16], large: &[u16; 512]) -> Vec<(u16, usize)> {
    let mut first = vec![
        u64::from((pattern >> 6) & 0b111),
        u64::from((pattern >> 3) & 0b111),
        u64::from(pattern & 0b111),
    ];
    let mut second = Vec::new();
    expand_grid(&first, &mut second, small, large);
    expand_grid(&second, &mut first, small, large);
    expand_grid(&first, &mut second, small, large);
    let mut counts = Vec::<(u16, usize)>::new();
    for ((&r1, &r2), &r3) in second.iter().zip(&second[1..]).zip(&second[2..]).step_by(3) {
        for shift in (0..9).step_by(3) {
            let key = u16::try_from(
                ((r1 >> shift) & 0b111) << 6 | ((r2 >> shift) & 0b111) << 3 | (r3 >> shift) & 0b111,
            )
            .unwrap();
            if let Some(count) = counts
                .iter_mut()
                .find_map(|t| (t.0 == key).then_some(&mut t.1))
            {
                *count += 1;
            } else {
                counts.push((key, 1));
            }
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
    ../.# => ##./#../...\n\
    .#./..#/### => #..#/..../..../#..#\
    ";

    fn print_grid(pattern: &[u64]) {
        let n = pattern.len();
        for &x in pattern {
            for i in 0..n {
                if (x >> i) & 1 == 0 {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }
        println!();
    }

    #[test]
    fn test_part_1() {
        let rules = parse(EXAMPLE).unwrap();
        let (small, large) = create_lookups(&rules);
        let mut pattern: Vec<u64> = vec![0b010, 0b001, 0b111];
        let mut next = Vec::new();
        print_grid(&pattern);
        for _ in 0..5 {
            expand_grid(&pattern, &mut next, &small, &large);
            (pattern, next) = (next, pattern);
            print_grid(&pattern);
        }
    }
}
