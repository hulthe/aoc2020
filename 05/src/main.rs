#![feature(test)]
extern crate test;

use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
pub struct Seat {
    pub column: usize,
    pub row: usize,
}

impl Seat {
    pub fn seat_id(&self) -> usize {
        self.column + 8 * self.row
    }
}

fn decode_seat(row: &str) -> Seat {
    let mut rows = 0..128;
    for letter in row.chars().take(7) {
        let diff = rows.end - rows.start;
        match letter {
            'B' => rows.start += diff / 2,
            'F' => rows.end -= diff / 2,
            _ => panic!("invalid row letter"),
        }
    }

    let mut cols = 0..8;
    for letter in row.chars().skip(7) {
        let diff = cols.end - cols.start;
        match letter {
            'R' => cols.start += diff / 2,
            'L' => cols.end -= diff / 2,
            _ => panic!("invalid col letter"),
        }
    }

    assert_eq!(rows.end - rows.start, 1);
    assert_eq!(cols.end - cols.start, 1);

    Seat {
        column: cols.start,
        row: rows.start,
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let seat = decode_seat(line);
            seat.seat_id()
        })
        .max()
        .unwrap_or(0)
}

pub fn part_2(input: &str) -> usize {
    let seats: HashSet<_> = input
        .lines()
        .map(|line| {
            let seat = decode_seat(line);
            seat.seat_id()
        })
        .collect();

    let missing: Vec<_> = (1..127)
        .flat_map(|row| (0..8).map(move |column| Seat { row, column }))
        .map(|seat| seat.seat_id())
        .filter(|seat_id| !seats.contains(seat_id))
        .filter(|seat_id| seats.contains(&(seat_id + 1)))
        .filter(|seat_id| seats.contains(&(seat_id - 1)))
        .collect();

    eprintln!("{:#?}", missing);
    assert_eq!(missing.len(), 1);
    missing[0]
}

fn main() {
    let input = include_str!("input");
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::{decode_seat, part_1, part_2};
    use test::{black_box, Bencher};

    #[test]
    fn test_decode() {
        let inputs = &[
            ("BFFFBBFRRR", (70, 7), 567),
            ("FFFBBBFRRR", (14, 7), 119),
            ("BBFFBBFRLL", (102, 4), 820),
        ];

        for (input, (row, column), seat_id) in inputs {
            let seat = decode_seat(input);

            assert_eq!(&seat.column, column);
            assert_eq!(&seat.row, row);
            assert_eq!(&seat.seat_id(), seat_id);
        }
    }

    #[bench]
    pub fn bench_part_1(b: &mut Bencher) {
        let input = include_str!("input");
        b.iter(|| part_1(black_box(input)));
    }

    #[bench]
    pub fn bench_part_2(b: &mut Bencher) {
        let input = include_str!("input");
        b.iter(|| part_2(black_box(input)));
    }
}
