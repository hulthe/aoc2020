struct Policy<'a> {
    min: usize,
    max: usize,
    pattern: &'a str,
    password: &'a str,
}

fn parse(input: &str) -> impl Iterator<Item = Policy> {
    input.lines().map(parse_line)
}

fn parse_line(input: &str) -> Policy {
    let (min, input) = input.split_once('-').unwrap();
    let (max, input) = input.split_once(' ').unwrap();
    let (pattern, password) = input.split_once(": ").unwrap();

    Policy {
        min: min.parse().unwrap(),
        max: max.parse().unwrap(),
        pattern,
        password,
    }
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .filter(|policy| {
            let count = policy.password.matches(policy.pattern).count();
            count >= policy.min && count <= policy.max
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .filter(|policy| {
            [policy.min, policy.max]
                .iter()
                .map(|&index| &policy.password[index - policy.pattern.len()..index])
                .filter(|&part| part == policy.pattern)
                .count()
                == 1
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 2);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 1);
    }
}
