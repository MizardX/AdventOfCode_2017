use std::num::ParseIntError;

#[aoc_generator(day2)]
fn parse(input: &str) -> Result<Vec<Vec<i64>>, ParseIntError> {
    input
        .lines()
        .map(|row| row.split_ascii_whitespace().map(str::parse).collect())
        .collect()
}

#[aoc(day2, part1)]
fn part_1(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|row| {
            let (min, max) = row.iter().fold((i64::MAX, i64::MIN), |(min, max), &x| {
                (min.min(x), max.max(x))
            });
            max - min
        })
        .sum()
}

#[aoc(day2, part2)]
fn part_2(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|row| {
            for (i, &x) in row.iter().enumerate() {
                for &y in &row[i+1..] {
                    if x % y == 0 {
                        return x / y;
                    } else if y % x == 0 {
                        return y / x;
                    }
                }
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
        5 1 9 5\n\
        7 5 3\n\
        2 4 6 8\n\
    ".trim_ascii();

    const EXAMPLE2: &str = "\
        5 9 2 8\n\
        9 4 7 3\n\
        3 8 6 5\n\
    ".trim_ascii();

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE1).unwrap();
        assert_eq!(result, [&[5,1,9,5][..],&[7,5,3][..],&[2,4,6,8][..]]);
    }

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE1).unwrap();
        let result = part_1(&input);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE2).unwrap();
        let result = part_2(&input);
        assert_eq!(result, 9);
    }
}