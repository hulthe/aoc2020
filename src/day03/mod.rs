#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Tree,
    Open,
}

#[derive(Debug)]
pub struct Map<'a> {
    pub width: usize,
    pub map: &'a [u8],
}

pub fn parse<'a>(input: &'a str) -> Map<'a> {
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
    fn get(&self, x: usize, y: usize) -> Option<Tile> {
        let match_char = |&c| match c {
            b'.' => Tile::Open,
            b'#' => Tile::Tree,
            t => panic!("invalid tile: '{}'", t as char),
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

pub fn part1(input: &str) -> usize {
    let map = parse(input);
    check_slope(&map, 3, 1)
}

pub fn part2(input: &str) -> usize {
    let map = parse(input);
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(step_x, step_y)| check_slope(&map, step_x, step_y))
        .product()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 7);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 336);
    }
}
