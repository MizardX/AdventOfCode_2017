#[aoc(day1, part1)]
fn part_1(input: &[u8]) -> u64 {
    let mut sum = 0;
    for (&a, &b) in input.iter().zip(input.iter().skip(1).chain([&input[0]])) {
        if a == b {
            sum += u64::from(a - b'0');
        }
    }
    sum
}

#[aoc(day1, part2)]
fn part_2(input: &[u8]) -> u64 {
    let mut sum = 0;
    for (&a, &b) in input.iter().zip(input.iter().cycle().skip(input.len()/2)) {
        if a == b {
            sum += u64::from(a - b'0');
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    
    #[test_case(b"1122" => 3)]
    #[test_case(b"1111" => 4)]
    #[test_case(b"1234" => 0)]
    #[test_case(b"91212129" => 9)]
    fn test_part_1(input: &[u8]) -> u64 {
        part_1(input)
    }

    #[test_case(b"1212" => 6)]
    #[test_case(b"1221" => 0)]
    #[test_case(b"123425" => 4)]
    #[test_case(b"123123" => 12)]
    #[test_case(b"12131415" => 4)]
    fn test_part_2(input: &[u8]) -> u64 {
        part_2(input)
    }
}