use std::collections::{HashMap, VecDeque};
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Program {
    name: usize,
    weight: u64,
    children: Vec<usize>,
}

impl Program {
    fn parse<'a>(
        s: &'a str,
        string_pool: &mut HashMap<&'a str, usize>,
    ) -> Result<Self, ParseError> {
        let (name, rest) = s.split_once(" (").ok_or(ParseError::SyntaxError)?;
        let next_index = string_pool.len();
        let name = *string_pool.entry(name).or_insert(next_index);

        let (weight, rest) = rest.split_once(')').ok_or(ParseError::SyntaxError)?;
        let weight = weight.parse()?;
        let mut children = Vec::new();
        if let Some(rest) = rest.strip_prefix(" -> ") {
            for word in rest.split(", ") {
                let next_index = string_pool.len();
                children.push(*string_pool.entry(word).or_insert(next_index));
            }
        }
        Ok(Self {
            name,
            weight,
            children,
        })
    }
}

struct Towers {
    string_pool: Vec<String>,
    programs: Vec<Program>,
}

impl FromStr for Towers {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lookup = HashMap::new();
        let mut programs = s
            .lines()
            .map(|line| Program::parse(line, &mut lookup))
            .collect::<Result<Vec<_>, _>>()?;
        programs.sort_unstable_by_key(|p| p.name);
        for (i, p) in programs.iter().enumerate() {
            assert_eq!(i, p.name);
        }
        let mut string_pool = vec![String::new(); lookup.len()];
        for (name, ix) in lookup {
            string_pool[ix] = name.to_string();
        }
        Ok(Self {
            string_pool,
            programs,
        })
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Result<Towers, ParseError> {
    input.parse()
}

#[aoc(day7, part1)]
fn part_1(towers: &Towers) -> String {
    // Find program that is not a child of any other
    let mut is_child = vec![false; towers.programs.len()];
    for program in &towers.programs {
        for &child in &program.children {
            is_child[child] = true;
        }
    }
    for (ix, name) in towers.string_pool.iter().enumerate() {
        if !is_child[ix] {
            return name.clone();
        }
    }
    String::new()
}

#[aoc(day7, part2)]
fn part_2(towers: &Towers) -> u64 {
    // Find the unique program that causes unbalance
    let total_weight = calculate_total_weight(towers);
    for program in &towers.programs {
        if program.children.is_empty() {
            continue;
        }
        if let Some((common_weight, unique_weight, unique_ix)) =
            find_unbalanced(&program.children, &total_weight)
        {
            // Check if the child itself is balanced
            let candidate = &towers.programs[unique_ix];
            if find_unbalanced(&candidate.children, &total_weight).is_none() {
                return candidate.weight + common_weight - unique_weight;
            }
        }
    }
    0
}

fn calculate_total_weight(towers: &Towers) -> Vec<u64> {
    let n = towers.programs.len();
    let mut total_weight = vec![0; n];
    let mut waiting_on = vec![vec![]; n];
    let mut queue: VecDeque<_> = (0..n).collect();

    'next_in_queue: while let Some(index) = queue.pop_front() {
        let mut sum = towers.programs[index].weight;
        for &child in &towers.programs[index].children {
            let child_weight = total_weight[child];
            if child_weight == 0 {
                waiting_on[child].push(index);
                continue 'next_in_queue;
            }
            sum += child_weight;
        }
        total_weight[index] = sum;
        queue.extend(waiting_on[index].drain(..));
    }
    total_weight
}

fn find_unbalanced(children: &[usize], total_weight: &[u64]) -> Option<(u64, u64, usize)> {
    let mut common_weight = None;
    let mut common_index = None;
    let mut common_count = 0_usize;
    let mut unique_weight = None;
    let mut unique_index = None;
    for &index in children {
        let w = total_weight[index];
        if let Some(common) = common_weight {
            if w == common {
                common_count += 1;
            } else if let Some(unique) = unique_weight {
                if w == unique && common_count == 1 {
                    // 'common' was actually the unique one
                    common_weight = Some(unique);
                    common_count = 2;
                    unique_weight = Some(common);
                    unique_index = common_index;
                } else {
                    // Either a third weight, or multiple of both weights.
                    panic!("Not a single unique weight");
                }
            } else {
                // First not equal to common; Assume unique
                unique_weight = Some(w);
                unique_index = Some(index);
            }
        } else {
            // First; assume common
            common_weight = Some(w);
            common_count = 1;
            common_index = Some(index);
        }
    }
    Some((common_weight?, unique_weight?, unique_index?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        pbga (66)\n\
        xhth (57)\n\
        ebii (61)\n\
        havc (66)\n\
        ktlj (57)\n\
        fwft (72) -> ktlj, cntj, xhth\n\
        qoyq (66)\n\
        padx (45) -> pbga, havc, qoyq\n\
        tknk (41) -> ugml, padx, fwft\n\
        jptl (61)\n\
        ugml (68) -> gyxo, ebii, jptl\n\
        gyxo (61)\n\
        cntj (57)\n\
    "
    .trim_ascii();

    #[test]
    fn test_parse() {
        const EMPTY: &[&str] = &[];
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(result.programs.len(), 13);
        let mut lookup = HashMap::new();
        for (ix, name) in result.string_pool.iter().enumerate() {
            lookup.insert(name.clone(), ix);
        }
        let expected_names = [
            "pbga", "xhth", "ebii", "havc", "ktlj", "fwft", "qoyq", "padx", "tknk", "jptl", "ugml",
            "gyxo", "cntj",
        ];
        let expected_weights = [66, 57, 61, 66, 57, 72, 66, 45, 41, 61, 68, 61, 57];
        let expected_children = [
            EMPTY,
            EMPTY,
            EMPTY,
            EMPTY,
            EMPTY,
            &["ktlj", "cntj", "xhth"][..],
            EMPTY,
            &["pbga", "havc", "qoyq"][..],
            &["ugml", "padx", "fwft"][..],
            EMPTY,
            &["gyxo", "ebii", "jptl"][..],
            EMPTY,
            EMPTY,
        ];
        for (exp_ix, &name) in expected_names.iter().enumerate() {
            let ix = *lookup.get(name).unwrap();
            assert_eq!(result.programs[ix].weight, expected_weights[exp_ix]);
            assert_eq!(
                result.programs[ix].children.len(),
                expected_children[exp_ix].len()
            );
            for (&child_ix, &child_name) in result.programs[ix]
                .children
                .iter()
                .zip(expected_children[exp_ix])
            {
                assert_eq!(result.string_pool[child_ix], child_name);
            }
        }
    }

    #[test]
    fn test_part_1() {
        let towers = parse(EXAMPLE).unwrap();
        let root_name = part_1(&towers);
        assert_eq!(root_name, "tknk");
    }

    #[test]
    fn test_part_2() {
        let towers = parse(EXAMPLE).unwrap();
        let updated_weight = part_2(&towers);
        assert_eq!(updated_weight, 60);
    }
}
