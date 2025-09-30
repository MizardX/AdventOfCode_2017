use std::num::ParseIntError;

#[aoc_generator(day5)]
fn parse(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day5, part1)]
fn part_1(input: &[isize]) -> usize {
    let mut offsets = input.to_vec();
    run(&mut offsets, false)
}

#[aoc(day5, part2)]
fn part_2(input: &[isize]) -> usize {
    let mut offsets = input.to_vec();
    run(&mut offsets, true)
}

fn run(offsets: &mut [isize], part2: bool) -> usize {
    let mut ip = 0_usize;
    let mut steps = 0;
    while let Some(&jump) = offsets.get(ip) {
        steps += 1;
        offsets[ip] = if part2 && jump >= 3 {
            jump - 1
        } else {
            jump + 1
        };
        let Some(new_ip) = ip.checked_add_signed(jump)else {
            break;
        };
        ip = new_ip;
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let mut offsets = [0, 3, 0, 1, -3];
        let result = run(&mut offsets, false);
        assert_eq!(result, 5);
        assert_eq!(offsets, [2, 5, 0, 1, -2]);
    }

    #[test]
    fn test_part_2() {
        let mut offsets = [0, 3, 0, 1, -3];
        let result = run(&mut offsets, true);
        assert_eq!(result, 10);
        assert_eq!(offsets, [2, 3, 2, 3, -1]);
    }
}
