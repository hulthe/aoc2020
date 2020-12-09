use std::collections::VecDeque;

fn parse<'a>(input: &'a str) -> impl Iterator<Item = usize> + 'a {
    input.lines().map(|line| line.parse().unwrap())
}

pub fn find_key<const N: usize>(numbers: &[usize]) -> usize {
    let mut buf = vec![0; N];

    // read preamble
    for (i, &num) in numbers.iter().take(N).enumerate() {
        buf[i] = num;
    }

    for (i, &num) in numbers.iter().skip(N).enumerate() {
        let i = i % buf.len();

        // check if the new number is the sum of two previous numbers
        let mut valid = false;
        for (k, &buf1) in buf.iter().enumerate() {
            for &buf2 in &buf[0..k] {
                if buf1 + buf2 == num {
                    valid = true;
                    break;
                }
            }
        }

        // if it wasn't, this number is the key
        if !valid {
            return num;
        }

        buf[i] = num;
    }

    panic!("solution not found");
}

const ACTUAL_N: usize = 25;

pub fn part1(input: &str) -> usize {
    let numbers: Vec<_> = parse(input).collect();
    find_key::<ACTUAL_N>(&numbers)
}

pub fn part2(input: &str) -> usize {
    let numbers: Vec<_> = parse(input).collect();
    let key = find_key::<ACTUAL_N>(&numbers);

    let mut sum = numbers[0];

    let mut parts = VecDeque::new();
    parts.push_back(numbers[0]);

    for &n in numbers.iter() {
        while sum > key {
            sum -= parts.pop_front().unwrap();
        }

        if sum == key {
            return parts.iter().min().unwrap() + parts.iter().max().unwrap();
        }

        parts.push_back(n);
        sum += n;
    }

    panic!("solution not found");
}

#[cfg(test)]
mod tests {
    use super::{find_key, parse};

    #[test]
    pub fn test_find_key() {
        let input = include_str!("test-input");
        let numbers: Vec<_> = parse(input).collect();
        assert_eq!(find_key::<5>(&numbers), 127);
    }
}
