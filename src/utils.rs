#[derive(Debug, Clone)]
pub struct KnotHasher<const N: usize = 256> {
    lengths: Vec<u8>,
    state: [u8; N],
    scratch: [u8; N],
    pos: usize,
    skip: usize,
}

impl<const N: usize> KnotHasher<N> {
    pub fn with_raw_lengths(lengths: &[u8]) -> Self {
        let mut state = [0; N];
        for (i, x) in state.iter_mut().enumerate() {
            *x = u8::try_from(i).unwrap();
        }
        let mut lengths_vec = Vec::with_capacity(lengths.len() + 5);
        lengths_vec.extend_from_slice(lengths);
        Self {
            lengths: lengths_vec,
            state,
            scratch: [0; N],
            pos: 0,
            skip: 0,
        }
    }

    pub fn new(lengths: &[u8]) -> Self {
        let mut hasher = Self::with_raw_lengths(lengths);
        hasher.lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
        hasher
    }

    pub fn reset(&mut self, lengths: &[u8]) {
        self.lengths.clear();
        self.lengths.reserve(lengths.len() + 5);
        self.lengths.extend_from_slice(lengths);
        self.lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
        for (i, x) in self.state.iter_mut().enumerate() {
            *x = u8::try_from(i).unwrap();
        }
        self.pos = 0;
        self.skip = 0;
    }

    pub fn scramble_once(&mut self) {
        for &len in &self.lengths {
            let len = len as usize;
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

    pub fn scramble_full(&mut self) {
        for _ in 0..64 {
            self.scramble_once();
        }
    }

    pub fn small_hash(&self) -> u16 {
        u16::from(self.state[0]) * u16::from(self.state[1])
    }

    pub fn write_hash<const N1: usize>(&self, hash: &mut [u8; N1]) {
        assert_eq!(N1 * 16, N, "N1 must be N/16");
        for (i, sum) in hash.iter_mut().enumerate() {
            *sum = 0;
            for &x in &self.state[i * 16..i * 16 + 16] {
                *sum ^= x;
            }
        }
    }

    pub fn large_hash(&self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut hash = [0; 16];
        self.write_hash(&mut hash);
        let mut res = Vec::with_capacity(N / 8);
        for b in hash {
            res.push(HEX[(b >> 4) as usize]);
            res.push(HEX[(b & 0xF) as usize]);
        }
        unsafe { String::from_utf8_unchecked(res) }
    }
}

#[derive(Debug, Clone, Copy)]
struct UnionFindNode {
    parent: usize,
    size: usize,
}

#[derive(Debug, Clone)]
pub struct UnionFind {
    nodes: Vec<UnionFindNode>,
    num_groups: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        let nodes = (0..size)
            .map(|parent| UnionFindNode { parent, size: 1 })
            .collect();
        Self {
            nodes,
            num_groups: size,
        }
    }

    pub fn find(&mut self, mut index: usize) -> usize {
        let mut parent = self.nodes[index].parent;
        while parent != index {
            let grand_parent = self.nodes[parent].parent;
            self.nodes[index].parent = grand_parent;
            index = grand_parent;
            parent = self.nodes[index].parent;
        }
        index
    }

    pub fn union(&mut self, mut index1: usize, mut index2: usize) -> bool {
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

    pub const fn num_groups(&self) -> usize {
        self.num_groups
    }
}
