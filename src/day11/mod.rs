use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Floor,
    Seat { occupied: bool },
}

pub type Map = HashMap<(i32, i32), Tile>;

pub fn parse(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .map(|(x, y, c)| {
            let tile = match c {
                '.' => Tile::Floor,
                '#' => Tile::Seat { occupied: true },
                'L' => Tile::Seat { occupied: false },
                _ => panic!("invalid tile char: '{}'", c),
            };
            ((x as i32, y as i32), tile)
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let mut tiles = parse(input);

    loop {
        let mut new_tiles = tiles.clone();
        new_tiles.par_iter_mut().for_each(|(&(x, y), tile)| {
            let occupied_adjacent_seats = (x.saturating_sub(1)..=x + 1)
                .flat_map(|x2| (y.saturating_sub(1)..=y + 1).map(move |y2| (x2, y2)))
                .filter(|&(x2, y2)| x2 != x || y2 != y)
                .filter_map(|(x2, y2)| tiles.get(&(x2, y2)).copied())
                .filter(|tile| match tile {
                    Tile::Seat { occupied } => *occupied,
                    _ => false,
                })
                .count();

            *tile = match *tile {
                Tile::Seat { occupied: false } if occupied_adjacent_seats == 0 => {
                    Tile::Seat { occupied: true }
                }
                Tile::Seat { occupied: true } if occupied_adjacent_seats >= 4 => {
                    Tile::Seat { occupied: false }
                }
                _ => *tile,
            };
        });

        if new_tiles == tiles {
            break;
        }

        tiles = new_tiles;
    }

    tiles
        .values()
        .filter(|tile| match tile {
            Tile::Seat { occupied } => *occupied,
            _ => false,
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    let mut tiles = parse(input);

    loop {
        let new_tiles = tiles
            .iter()
            .map(|(&(x, y), &tile)| {
                let dirs = [
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                    (-1, -1),
                ];

                let mut occupied_adjacent_seats = 0;
                for &(dx, dy) in dirs.iter() {
                    let mut cx = x + dx;
                    let mut cy = y + dy;

                    while let Some(tile) = tiles.get(&(cx, cy)).copied() {
                        if let Tile::Seat { occupied } = tile {
                            if occupied {
                                occupied_adjacent_seats += 1;
                            }
                            break;
                        }

                        cx += dx;
                        cy += dy;
                    }
                }

                let tile = match tile {
                    Tile::Seat { occupied: false } if occupied_adjacent_seats == 0 => {
                        Tile::Seat { occupied: true }
                    }
                    Tile::Seat { occupied: true } if occupied_adjacent_seats >= 5 => {
                        Tile::Seat { occupied: false }
                    }
                    _ => tile,
                };

                ((x, y), tile)
            })
            .collect();

        if new_tiles == tiles {
            break;
        }

        tiles = new_tiles;
    }

    tiles
        .values()
        .filter(|tile| match tile {
            Tile::Seat { occupied } => *occupied,
            _ => false,
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 37);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 26);
    }
}
