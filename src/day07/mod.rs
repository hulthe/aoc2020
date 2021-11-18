use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref LINE_RGX: Regex = Regex::new(
        r#"(?x)
        (?P<parent> \w+ \s \w+ )
        \s bags \s contain
        (?P<contains> .+ )
        "#
    )
    .unwrap();
    static ref CONTAINS_RGX: Regex = Regex::new(
        r#"(?x)
        \s (?P<num> \d+ )
        \s (?P<child> \w+ \s \w+ )
        \s bag s? [\.,]
        "#,
    )
    .unwrap();
}

#[allow(dead_code)]
pub fn parse(_input: &str) {}

/// Map every (parent, num, child) combination of the input into a closure
fn parse_into<'a, F>(input: &'a str, mut into: F)
where
    F: FnMut(&'a str, usize, &'a str),
{
    for capture in input.lines().flat_map(|line| LINE_RGX.captures(line)) {
        let parent = capture.name("parent").unwrap().as_str();
        let contains = capture.name("contains").unwrap().as_str();

        CONTAINS_RGX.captures_iter(contains).for_each(|sub| {
            let num = sub.name("num").unwrap().as_str().parse().unwrap();
            let child = sub.name("child").unwrap().as_str();

            into(parent, num, child);
        });
    }
}

/// Child bags to all possible parent bags
pub fn parse1(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut map = HashMap::new();

    parse_into(input, |parent, _num, child| {
        map.entry(child).or_insert(vec![]).push(parent);
    });

    map
}

/// Map parent bags to all possible (count, child):s
pub fn parse2(input: &str) -> HashMap<&str, Vec<(usize, &str)>> {
    let mut map = HashMap::new();

    parse_into(input, |parent, num, child| {
        map.entry(parent).or_insert(vec![]).push((num, child));
    });

    map
}

const MY_BAG: &str = "shiny gold";

pub fn part1(input: &str) -> usize {
    let map = parse1(input);

    let mut possible_parent_colors = 0;

    let mut visited = HashSet::new();
    let mut next: Vec<&[&str]> = vec![map.get(MY_BAG).map(|v| &v[..]).unwrap_or(&[])];

    while let Some(n) = next.pop() {
        for color in n {
            if visited.contains(color) {
                continue;
            }

            visited.insert(color);

            possible_parent_colors += 1;
            if let Some(bags) = map.get(color) {
                next.push(bags);
            }
        }
    }

    possible_parent_colors
}

pub fn part2(input: &str) -> usize {
    let map = parse2(input);

    let mut count = 0;

    let mut next: Vec<(usize, &[(usize, &str)])> =
        vec![map.get(MY_BAG).map(|v| (1, &v[..])).unwrap()];

    while let Some((mul, n)) = next.pop() {
        for (num, color) in n {
            count += num * mul;
            if let Some(bags) = map.get(color) {
                next.push((num * mul, bags));
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input1");
        assert_eq!(part1(input), 4);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input2");
        assert_eq!(part2(input), 126);
    }
}
