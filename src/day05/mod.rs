use std::str::from_utf8;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Seat {
    id: usize,
}

impl Seat {
    fn decode(line: &str) -> Seat {
        let mut ascii = [0u8; 10];
        ascii.clone_from_slice(line.as_bytes());

        for i in 0..10 {
            ascii[i] = match ascii[i] {
                b'B' | b'R' => b'1',
                b'F' | b'L' => b'0',
                _ => panic!("invalid char"),
            };
        }

        Seat {
            id: usize::from_str_radix(from_utf8(&ascii).unwrap(), 2).unwrap(),
        }
    }
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| Seat::decode(line).id)
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let mut seats: Vec<_> = input.lines().map(|line| Seat::decode(line)).collect();
    seats.sort();

    let mut expected: usize = seats[0].id;
    seats
        .iter()
        .filter_map(|seat| {
            if seat.id != expected {
                Some(expected)
            } else {
                expected = seat.id + 1;
                None
            }
        })
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::Seat;

    #[test]
    fn test_decode() {
        let inputs = &[
            ("BFFFBBFRRR", 567),
            ("FFFBBBFRRR", 119),
            ("BBFFBBFRLL", 820),
        ];

        for (input, seat_id) in inputs {
            let seat = Seat::decode(input);

            assert_eq!(&seat.id, seat_id);
        }
    }
}
