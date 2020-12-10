use std::collections::{HashMap, HashSet};

pub fn parse(input: &str) -> HashSet<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(input: &str) -> u64 {
    let adapters = parse(input);

    let max_joltage = *adapters.iter().max().unwrap();

    let mut diffs1 = 0;
    //let mut diffs2 = 0;
    let mut diffs3 = 0;
    let mut current_joltage = 0;
    while current_joltage < max_joltage {
        if let Some(next) = adapters.get(&(current_joltage + 1)) {
            diffs1 += 1;
            current_joltage = *next;
        } else if let Some(next) = adapters.get(&(current_joltage + 2)) {
            //diffs2 += 1;
            current_joltage = *next;
        } else if let Some(next) = adapters.get(&(current_joltage + 3)) {
            diffs3 += 1;
            current_joltage = *next;
        } else {
            panic!("no next step from {}", current_joltage);
        }
    }

    // final step: biggest adapter -> device
    diffs3 += 1;

    diffs1 * diffs3
}

pub fn part2(input: &str) -> u64 {
    let adapters = parse(input);

    let max_joltage = *adapters.iter().max().unwrap();

    let mut sets: HashMap<u64, u64> = HashMap::new();
    sets.insert(max_joltage, 1);

    // The set of all possible adapters that plugs into _this_ adapter
    // is equal to the sum of the sets of possible adapters of joltage
    // +1, +2 and +3. Therefore we start from the biggest adapter and
    // work our way down.
    for adapter in (0..max_joltage).rev() {
        if adapters.contains(&adapter) || adapter == 0 {
            let subset_count = (1..=3)
                .filter_map(|step| sets.get(&(adapter + step)).copied())
                .sum();
            sets.insert(adapter, subset_count);
        }
    }

    *sets.get(&0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 22 * 10);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 19208);

        let input = include_str!("test-input2");
        assert_eq!(part2(input), 8);
    }
}
