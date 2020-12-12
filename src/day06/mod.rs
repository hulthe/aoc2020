pub fn parse(_input: &str) -> () {}

pub fn part1(input: &str) -> usize {
    // Buffer to store results of each group
    let mut results = [0usize; 1024];

    rayon::scope(|s| {
        let mut results = &mut results[..];

        for group in input.split("\n\n") {
            // Mutably split out the first element and use it to store the result of the group
            let (group_count, tail) = results.split_first_mut().expect("buffer too small");
            results = tail;

            // Spawn a thread to do the work
            s.spawn(move |_| {
                // Count which ascii chars appear anywhere in the group
                let mut chars = [false; 256];
                for &c in group.as_bytes().iter().filter(|c| !c.is_ascii_whitespace()) {
                    chars[c as usize] = true;
                }
                *group_count = chars.iter().filter(|&b| *b).count();
            })
        }
    });

    results.iter().copied().sum()
}

pub fn part2(input: &str) -> usize {
    // Buffer to store results of each group
    let mut results = [0usize; 1024];

    rayon::scope(|s| {
        let mut results = &mut results[..];

        for group in input.split("\n\n") {
            // Mutably split out the first element and use it to store the result of the group
            let (group_count, tail) = results.split_first_mut().expect("buffer too small");
            results = tail;

            // Spawn a thread to do the work
            s.spawn(move |_| {
                let mut group_chars = [true; 256];

                group.lines().for_each(|person| {
                    // Count which ascii chars appear in this persons answer
                    let mut person_chars = [false; 256];
                    person
                        .as_bytes()
                        .iter()
                        .filter(|c| !c.is_ascii_whitespace())
                        .for_each(|&c| person_chars[c as usize] = true);

                    // Intersect this persons answers with the rest of the groups
                    for i in 0..group_chars.len() {
                        if !person_chars[i] {
                            group_chars[i] = false;
                        }
                    }
                });

                *group_count = group_chars.iter().filter(|&b| *b).count();
            })
        }
    });

    results.iter().copied().sum()
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
