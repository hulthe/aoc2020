use std::str::from_utf8;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Seat {
    id: usize,
}

impl Seat {
    fn decode(line: &str) -> Seat {
        let mut ascii = [0u8; 10];
        ascii.clone_from_slice(line.as_bytes());

        // interpret seat code as a binary number
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

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = usize> + 'a {
    input.lines().map(|line| Seat::decode(line).id)
}

pub fn part1(input: &str) -> usize {
    // get the biggest occupied seat id
    parse(input).max().unwrap()
}

pub fn part2(input: &str) -> usize {
    // all possible seat ids, and whether they are occupied
    let mut seats_occupied = [false; 1 << 10];

    // populate seats_occupied
    parse(input).for_each(|id| seats_occupied[id] = true);

    seats_occupied
        .iter()
        .enumerate()
        .filter(|(_, &occupied)| !occupied) // look for empty seats
        .map(|(id, _)| id)
        .filter(|id| seats_occupied[id + 1]) // check that the seats next to it are occupied
        .filter(|id| seats_occupied[id - 1])
        .next() // take the first match
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
