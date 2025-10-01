use std::num::ParseIntError;

#[aoc_generator(day17)]
fn parse(s: &str) -> Result<usize, ParseIntError> {
    s.parse()
}

#[aoc(day17, part1)]
#[expect(clippy::trivially_copy_pass_by_ref, reason = "AOC lib")]
fn part_1(&step: &usize) -> usize {
    let mut circular_buffer = Vec::with_capacity(2017);
    circular_buffer.push(0);
    let mut pos = 0;
    for new_value in 1..=2017 {
        pos = (pos + step) % circular_buffer.len() + 1;
        circular_buffer.insert(pos, new_value);
    }
    circular_buffer[pos + 1]
}

#[aoc(day17, part2)]
#[expect(clippy::trivially_copy_pass_by_ref, reason = "AOC lib")]
fn part_2(&step: &usize) -> usize {
    let mut pos = 0;
    let mut value_after_zero = 0;
    for t in 1..=50_000_000 {
        pos = (pos + step) % t + 1;
        if pos == 1 {
            value_after_zero = t;
        }
    }
    value_after_zero
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1(&3);
        assert_eq!(result, 638);
    }
}
