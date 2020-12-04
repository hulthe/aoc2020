#![feature(test)]
extern crate test;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Tree,
    Open,
}

#[derive(Debug)]
struct Map<'a> {
    pub width: usize,
    pub map: &'a [u8],
}

fn parse_map<'a>(input: &'a str) -> Map<'a> {
    let width = input
        .lines()
        .map(|line| line.chars().count())
        .next()
        .unwrap();

    Map {
        map: input.as_bytes(),
        width,
    }
}

impl Map<'_> {
    pub fn get(&self, x: usize, y: usize) -> Option<Tile> {
        let match_char = |&c| match c {
            b'.' => Tile::Open,
            b'#' => Tile::Tree,
            _ => panic!("invalid tile"),
        };

        let x = x % self.width;
        let i = x + y * (self.width + 1); // +1 to account for newline character
        self.map.get(i).map(match_char)
    }
}

fn check_slope(map: &Map, step_x: usize, step_y: usize) -> usize {
    let xs = (0..).step_by(step_x);
    let ys = (0..).step_by(step_y);

    let poss = xs.zip(ys);

    poss.skip(1)
        .map(|(x, y)| map.get(x, y))
        .take_while(|tile| tile.is_some())
        .map(|tile| tile.unwrap())
        .filter(|&tile| tile == Tile::Tree)
        .count()
}

fn part_1(input: &str) -> usize {
    let map = parse_map(input);
    check_slope(&map, 3, 1)
}

fn part_2(input: &str) -> usize {
    let map = parse_map(input);
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(step_x, step_y)| check_slope(&map, step_x, step_y))
        .product()
}

fn main() {
    let input = include_str!("input");

    println!("part1: {}", part_1(input));
    println!("part2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::{parse_map, part_1, part_2};
    use test::{black_box, Bencher};

    #[test]
    pub fn test_part_1() {
        let input = include_str!("test-input");
        assert_eq!(part_1(input), 7);
    }

    #[test]
    pub fn test_part_2() {
        let input = include_str!("test-input");
        assert_eq!(part_2(input), 336);
    }

    #[bench]
    pub fn bench_parse(b: &mut Bencher) {
        let input = include_str!("input");
        b.iter(|| parse_map(black_box(input)));
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
