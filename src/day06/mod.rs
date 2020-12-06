use std::collections::HashSet;
use std::ops::BitAnd;

pub fn part1(input: &str) -> usize {
    let mut set = HashSet::new();
    input
        .split("\n\n")
        .map(|group| {
            set.clear();
            group.chars().filter(|c| !c.is_whitespace()).for_each(|c| {
                set.insert(c);
            });
            set.len()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| {
                    person
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect::<HashSet<char>>()
                })
                .fold_first(|a, b| a.bitand(&b))
                .unwrap()
        })
        .map(|set| set.iter().count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 11);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 6);
    }
}
