#[aoc(day10, part1)]
fn part_1(input: &str) -> u16 {
    let lengths = input
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut hasher = KnotHasher::<256>::new(&lengths);
    hasher.scramble();
    hasher.small_hash()
}

#[aoc(day10, part2)]
fn part_2(input: &str) -> String {
    let mut lengths = input.bytes().map(usize::from).collect::<Vec<_>>();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
    let mut hasher = KnotHasher::<256>::new(&lengths);
    for _ in 0..64 {
        hasher.scramble();
    }
    hasher.large_hash()
}

#[derive(Debug, Clone)]
struct KnotHasher<'a, const N: usize = 256> {
    lengths: &'a [usize],
    state: [u8; N],
    scratch: [u8; N],
    pos: usize,
    skip: usize,
}

impl<'a, const N: usize> KnotHasher<'a, N> {
    fn new(lengths: &'a [usize]) -> Self {
        let mut state = [0; N];
        for (i, x) in state.iter_mut().enumerate() {
            *x = u8::try_from(i).unwrap();
        }
        Self {
            lengths,
            state,
            scratch: [0; N],
            pos: 0,
            skip: 0,
        }
    }

    fn scramble(&mut self) {
        for &len in self.lengths {
            if self.pos + len > N {
                let a = self.pos + len - N;
                let b = N - self.pos;
                // Copy data to buf to reverse it
                // state          buf            buf            state
                // [tuv|..|yz] -> [yz|tuv|..] -> [vu|tzy|..] -> [tzy|..|vu]
                //    a^  ^pos      b^   ^len      b^   ^len       a^  ^pos
                self.scratch[..b].copy_from_slice(&self.state[self.pos..]);
                self.scratch[b..len].copy_from_slice(&self.state[..a]);
                self.scratch[..len].reverse();
                self.state[self.pos..].copy_from_slice(&self.scratch[..b]);
                self.state[..a].copy_from_slice(&self.scratch[b..len]);
            } else {
                self.state[self.pos..self.pos + len].reverse();
            }
            self.pos = (self.pos + len + self.skip) % N;
            self.skip += 1;
        }
    }

    fn small_hash(&self) -> u16 {
        u16::from(self.state[0]) * u16::from(self.state[1])
    }

    fn large_hash(&self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut res = Vec::with_capacity(N * 2);
        for i in (0..N).step_by(16) {
            let mut sum = 0;
            for &x in &self.state[i..i + 16] {
                sum ^= x;
            }
            res.push(HEX[(sum >> 4) as usize]);
            res.push(HEX[(sum & 0xF) as usize]);
        }
        unsafe { String::from_utf8_unchecked(res) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_part_1() {
        let mut hasher = KnotHasher::<5>::new(&[3, 4, 1, 5]);
        hasher.scramble();
        let result = hasher.small_hash();
        assert_eq!(result, 12);
    }

    #[test_case("" => "a2582a3a0e66e6e86e3812dcb672a272")]
    #[test_case("AoC 2017" => "33efeb34ea91902bb2f59c9920caa6cd")]
    #[test_case("1,2,3" => "3efbe78a8d82f29979031a4aa0b16a9d")]
    #[test_case("1,2,4" => "63960835bcdc130f0b66d7ff4f6a5a8e")]
    fn test_part_2(input: &str) -> String {
        part_2(input)
    }
}
