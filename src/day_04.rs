use std::collections::HashSet;

#[aoc(day4, part1)]
fn part_1(input: &str) -> usize {
    let mut seen = HashSet::new();
    input
        .lines()
        .filter(|line| unique_words(line, &mut seen))
        .count()
}

fn unique_words<'a>(passphrase: &'a str, seen: &mut HashSet<&'a str>) -> bool {
    seen.clear();
    passphrase.split(' ').all(|word| seen.insert(word))
}

#[aoc(day4, part2)]
fn part_2(input: &str) -> usize {
    let mut seen = HashSet::new();
    input
        .lines()
        .filter(|line| unique_palindromes(line, &mut seen))
        .count()
}

fn unique_palindromes(passphrase: &str, seen: &mut HashSet<[u8; 26]>) -> bool {
    seen.clear();
    passphrase.split(' ').all(|word| {
        let mut freq = [0_u8; 26];
        for ch in word.bytes() {
            freq[(ch - b'a') as usize] += 1;
        }
        seen.insert(freq)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("aa bb cc dd ee" => true)]
    #[test_case("aa bb cc dd aa" => false)]
    #[test_case("aa bb cc dd aaa" => true)]
    fn test_unique_words(line: &str) -> bool {
        unique_words(line, &mut HashSet::new())
    }

    #[test_case("abcde fghij" => true)]
    #[test_case("abcde xyz ecdab" => false)]
    #[test_case("a ab abc abd abf abj" => true)]
    #[test_case("iiii oiii ooii oooi oooo" => true)]
    #[test_case("oiii ioii iioi iiio" => false)]
    fn test_unique_palindromes(line: &str) -> bool {
        unique_palindromes(line, &mut HashSet::new())
    }
}
