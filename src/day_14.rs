use crate::utils::{KnotHasher, UnionFind};
use std::fmt::Write;

#[aoc(day14, part1)]
fn part_1(input: &str) -> u32 {
    let mut key = String::with_capacity(input.len() + 4);
    key.push_str(input);
    key.push('-');
    let mut hasher = KnotHasher::<256>::new(&[]);
    let prefix_len = key.len();
    let mut total_bits = 0;
    for r in 0..128 {
        key.truncate(prefix_len);
        write!(&mut key, "{r}").unwrap();
        hasher.reset(key.as_bytes());
        hasher.scramble_full();

        let mut row_hash = [0; 16];
        hasher.write_hash(&mut row_hash);
        total_bits += row_hash.into_iter().map(u8::count_ones).sum::<u32>();
    }
    total_bits
}

#[aoc(day14, part2)]
fn part_2(input: &str) -> usize {
    const OUTSIDE: usize = 128 * 128;
    const STRIDE: usize = 128;
    let mut uf = UnionFind::new(128 * 128 + 1);
    let mut key = String::with_capacity(input.len() + 4);
    key.push_str(input);
    key.push('-');
    let mut hasher = KnotHasher::<256>::new(&[]);
    let prefix_len = key.len();
    let mut prev_hash = None::<[u8; 16]>;
    for r in 0..128 {
        key.truncate(prefix_len);
        write!(&mut key, "{r}").unwrap();
        hasher.reset(key.as_bytes());
        hasher.scramble_full();

        let mut row_hash = [0; 16];
        hasher.write_hash(&mut row_hash);

        let mut prev_cell = false;
        for (c, cell) in BitIterator::new(&row_hash).enumerate() {
            if !cell {
                uf.union(OUTSIDE, r * STRIDE + c);
            } else if prev_cell {
                uf.union(r * STRIDE + c - 1, r * STRIDE + c);
            }
            prev_cell = cell;
        }
        if let Some(above) = prev_hash {
            for (c, (cell1, cell2)) in BitIterator::new(&above)
                .zip(BitIterator::new(&row_hash))
                .enumerate()
            {
                if cell1 && cell2 {
                    uf.union((r - 1) * STRIDE + c, r * STRIDE + c);
                }
            }
        }
        prev_hash = Some(row_hash);
    }
    uf.num_groups() - 1
}

#[derive(Debug, Clone)]
struct BitIterator<'a> {
    bytes: &'a [u8],
    index: usize,
    bit: u8,
}

impl<'a> BitIterator<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            index: 0,
            bit: 0,
        }
    }
}

impl Iterator for BitIterator<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.bytes.len() {
            return None;
        }
        let item = (self.bytes[self.index] >> (7 - self.bit)) & 1 != 0;
        self.bit += 1;
        if self.bit == 8 {
            self.bit = 0;
            self.index += 1;
        }
        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_iterator() {
        const T: bool = true;
        const F: bool = false;
        let data = b"\xa0\xc2\x01\x70";
        let bits = BitIterator::new(data).collect::<Vec<_>>();
        assert_eq!(
            &bits[..32],
            &[
                T, F, T, F, F, F, F, F, T, T, F, F, F, F, T, F, F, F, F, F, F, F, F, T, F, T, T, T,
                F, F, F, F
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let result = part_1("flqrgnkx");
        assert_eq!(result, 8108);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("flqrgnkx");
        assert_eq!(result, 1242);
    }
}
