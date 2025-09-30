#[derive(Debug, Clone, Copy)]
enum State {
    Normal,
    String,
    Escaped,
}

#[aoc(day9, part1)]
fn part_1(input: &[u8]) -> usize {
    measure(input).0
}

#[aoc(day9, part2)]
fn part_2(input: &[u8]) -> usize {
    measure(input).1
}

fn measure(input: &[u8]) -> (usize, usize) {
    let mut depth = 0;
    let mut score = 0;
    let mut count = 0;
    let mut state = State::Normal;
    for ch in input {
        state = match (state, ch) {
            (State::Normal, b'{') => {
                depth += 1;
                score += depth;
                State::Normal
            }
            (State::Normal, b'}') => {
                depth -= 1;
                State::Normal
            }
            (State::Normal, b'<') | (State::Escaped, _) => State::String,
            (State::Normal, _) | (State::String, b'>') => State::Normal,
            (State::String, b'!') => State::Escaped,
            (State::String, _) => {
                count += 1;
                State::String
            }
        };
    }
    (score, count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(b"{}" => 1)]
    #[test_case(b"{{{}}}" => 6)]
    #[test_case(b"{{},{}}" => 5)]
    #[test_case(b"{{{},{},{{}}}}" => 16)]
    #[test_case(b"{<a>,<a>,<a>,<a>}" => 1)]
    #[test_case(b"{{<ab>},{<ab>},{<ab>},{<ab>}}" => 9)]
    #[test_case(b"{{<!!>},{<!!>},{<!!>},{<!!>}}" => 9)]
    #[test_case(b"{{<a!>},{<a!>},{<a!>},{<ab>}}" => 3)]
    fn test_part_1(input: &[u8]) -> usize {
        part_1(input)
    }

    #[test_case(b"<>" => 0; "ltgt")]
    #[test_case(b"<random characters>" => 17)]
    #[test_case(b"<<<<>" => 3)]
    #[test_case(b"<{!>}>" => 2)]
    #[test_case(b"<!!>" => 0; "ltbangbanggt")]
    #[test_case(b"<!!!>>" => 0; "ltbangbangbanggtgt")]
    #[test_case(b"<{o\"i!a,<{i<a>" => 10)]
    fn test_part_2(input: &[u8]) -> usize {
        part_2(input)
    }
}
