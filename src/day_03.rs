use std::{collections::HashMap, num::ParseIntError};

#[aoc_generator(day3)]
fn parse(input: &str) -> Result<u64, ParseIntError> {
    input.parse()
}

#[aoc(day3, part1)]
#[expect(clippy::trivially_copy_pass_by_ref, reason = "AOC library")]
fn part_1(&input: &u64) -> u64 {
    let (x, y) = get_position(input);
    x.unsigned_abs() + y.unsigned_abs()
}

fn get_position(value: u64) -> (i64, i64) {
    let ring = i64::try_from((value - 1).isqrt().div_ceil(2)).unwrap();
    let value = i64::try_from(value).unwrap();
    // +x is right, +y is up
    if value <= (4 * ring - 2) * ring + 1 {
        // Right edge
        let x = ring;
        let y = value - ((4 * ring - 3) * ring + 1);
        (x, y)
    } else if value <= 4 * ring * ring + 1 {
        // Top edge
        let x = ((4 * ring - 1) * ring + 1) - value;
        let y = ring;
        (x, y)
    } else if value <= (4 * ring + 2) * ring + 1 {
        let x = -ring;
        let y = ((4 * ring + 1) * ring + 1) - value;
        (x, y)
    } else {
        let x = value - ((4 * ring + 3) * ring + 1);
        let y = -ring;
        (x, y)
    }
}

#[aoc(day3, part2)]
#[expect(clippy::trivially_copy_pass_by_ref, reason = "AOC library")]
fn part_2(&input: &u64) -> u64 {
    let mut values = HashMap::new();
    values.insert((0, 0), 1);
    for n in 2.. {
        let (x, y) = get_position(n);
        let mut sum = 0;
        for x1 in x - 1..=x + 1 {
            for y1 in y - 1..=y + 1 {
                if (x1, y1) != (x, y) {
                    sum += values.get(&(x1, y1)).copied().unwrap_or(0);
                }
            }
        }
        if sum > input {
            return sum;
        }
        values.insert((x, y), sum);
    }
    unreachable!("Overflow?")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_get_position() {
        // +x is right, +y is up
        // +r is down, +c is right
        let expected = [[5, 4, 3], [6, 1, 2], [7, 8, 9]];
        for (r, row) in expected.into_iter().enumerate() {
            let y = 1 - i64::try_from(r).unwrap();
            for (c, cell) in row.into_iter().enumerate() {
                let x = i64::try_from(c).unwrap() - 1;
                assert_eq!(get_position(cell), (x, y), "{cell}");
            }
        }
    }


    #[test_case(1 => 0)]
    #[test_case(12 => 3)]
    #[test_case(23 => 2)]
    #[test_case(1024 => 31)]
    fn test_part_1(input: u64) -> u64 {
        part_1(&input)
    }
}
