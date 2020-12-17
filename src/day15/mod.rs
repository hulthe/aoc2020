pub fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|num| num.parse().expect("failed to parse number"))
        .collect()
}
struct VecMap<T> {
    buffer: Vec<Option<T>>,
}

#[allow(dead_code)]
impl<T: Copy> VecMap<T> {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(cap),
        }
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.buffer.len() {
            return None;
        }
        self.buffer[index]
    }

    pub fn insert(&mut self, index: usize, value: T) -> Option<T> {
        if index >= self.buffer.len() {
            self.buffer.resize_with(index + 1, || None);
        }

        let old = self.buffer[index];
        self.buffer[index] = Some(value);
        old
    }
}

fn nth_number_spoken(input: &str, n: usize) -> usize {
    let starting_numbers = parse(input);

    let mut spoken = VecMap::with_capacity(65536);

    let mut most_recently_spoken = 0;
    let mut first_time = true;
    let mut previous_age = 0;

    for (i, &num) in starting_numbers.iter().enumerate() {
        spoken.insert(num, i);
        most_recently_spoken = num;
    }

    for i in starting_numbers.len()..n {
        // if the previous number had not been spoken before
        let speak = if first_time {
            0 // speak 0
        } else {
            // else speak the age of the number
            previous_age
        };

        if let Some(spoken_at) = spoken.insert(speak, i) {
            first_time = false;
            previous_age = (i - spoken_at) as usize;
        } else {
            first_time = true;
        }
        most_recently_spoken = speak;
    }

    most_recently_spoken
}

pub fn part1(input: &str) -> usize {
    nth_number_spoken(input, 2020)
}

pub fn part2(input: &str) -> usize {
    nth_number_spoken(input, 30_000_000)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_DATA: &[(&str, usize)] = &[
        ("0,3,6", 436),
        ("1,3,2", 1),
        ("2,1,3", 10),
        ("1,2,3", 27),
        ("2,3,1", 78),
        ("3,2,1", 438),
        ("3,1,2", 1836),
    ];

    #[test]
    pub fn test_part1() {
        for &(input, output) in TEST_DATA {
            assert_eq!(part1(input), output, "input was: {}", input);
        }
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 1337);
    }
}
