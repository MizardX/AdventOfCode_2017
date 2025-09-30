use std::num::ParseIntError;

#[aoc_generator(day6)]
fn parse(input: &str) -> Result<Vec<u8>, ParseIntError> {
    input.split_ascii_whitespace().map(str::parse).collect()
}

#[aoc(day6, part1)]
fn part_1(input: &[u8]) -> usize {
    let (len, offset) = if input.len() == 4 {
        find_loop::<4>(input.try_into().unwrap())
    } else {
        find_loop::<16>(input.try_into().unwrap())
    };
    len + offset
}
#[aoc(day6, part2)]
fn part_2(input: &[u8]) -> usize {
    if input.len() == 4 {
        find_loop::<4>(input.try_into().unwrap()).0
    } else {
        find_loop::<16>(input.try_into().unwrap()).0
    }
}

/// Brent's algorithm
fn find_loop<const N: usize>(start: [u8; N]) -> (usize, usize) {
    let mut slow = start;
    let mut fast = start;
    step(&mut fast);

    let mut power = 1;
    let mut length = 1;

    while slow != fast {
        if length == power {
            slow = fast;
            power *= 2;
            length = 0;
        }
        step(&mut fast);
        length += 1;
    }

    slow = start;
    fast = start;
    for _ in 0..length {
        step(&mut fast);
    }

    let mut offset = 0;
    while slow != fast {
        step(&mut fast);
        step(&mut slow);
        offset += 1;
    }

    (length, offset)
}

fn step<const N: usize>(state: &mut [u8; N]) {
    let mut max_index = 0;
    let mut max_value = state[0];
    let len = state.len();
    for (index, &value) in state.iter().enumerate() {
        if value > max_value {
            max_value = value;
            max_index = index;
        }
    }
    state[max_index] = 0;
    for index in max_index + 1..=max_index + max_value as usize {
        state[index % len] += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case([0,2,7,0] => [2,4,1,2])]
    #[test_case([2,4,1,2] => [3,1,2,3])]
    #[test_case([3,1,2,3] => [0,2,3,4])]
    #[test_case([0,2,3,4] => [1,3,4,1])]
    #[test_case([1,3,4,1] => [2,4,1,2])]
    fn test_step(mut state: [u8; 4]) -> [u8; 4] {
        step(&mut state);
        state
    }

    #[test]
    fn test_find_loop() {
        let state = [0, 2, 7, 0];
        let (len, offset) = find_loop(state);
        assert_eq!(len + offset, 5);
        assert_eq!(len, 4);
    }
}
