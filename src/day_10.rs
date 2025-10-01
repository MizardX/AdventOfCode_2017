use crate::utils::KnotHasher;

#[aoc(day10, part1)]
fn part_1(input: &str) -> u16 {
    let lengths = input
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut hasher = KnotHasher::<256>::with_raw_lengths(&lengths);
    hasher.scramble_once();
    hasher.small_hash()
}

#[aoc(day10, part2)]
fn part_2(input: &str) -> String {
    let lengths = input.bytes().collect::<Vec<_>>();
    let mut hasher = KnotHasher::<256>::new(&lengths);
    hasher.scramble_full();
    hasher.large_hash()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_part_1() {
        let mut hasher = KnotHasher::<5>::new(&[3, 4, 1, 5]);
        hasher.scramble_once();
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
