#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    value: i32,
    movement: Movement,
}

#[derive(Clone, Copy, Debug)]
pub enum Movement {
    Direction(Direction),
    Left,
    Right,
    Forward,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = Instruction> + 'a {
    input
        .lines()
        .map(|line| line.split_at(1))
        .map(|(movement, value)| {
            let movement = match movement {
                "N" => Movement::Direction(Direction::North),
                "S" => Movement::Direction(Direction::South),
                "W" => Movement::Direction(Direction::West),
                "E" => Movement::Direction(Direction::East),
                "L" => Movement::Left,
                "R" => Movement::Right,
                "F" => Movement::Forward,
                _ => panic!("non-recognised instruction: \"{}\"", movement),
            };

            Instruction {
                value: value.parse().unwrap(),
                movement,
            }
        })
}

impl Direction {
    pub fn as_delta(&self) -> (i32, i32) {
        match self {
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
            Direction::North => (0, 1),
            Direction::South => (0, -1),
        }
    }

    pub fn rotate(&self, mut degrees: i32) -> Self {
        while degrees < 0 {
            degrees = 360 + degrees;
        }

        let mut d = *self;
        for _ in 0..(degrees / 90) {
            d = match d {
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::North => Direction::East,
            };
        }

        d
    }
}

pub fn part1(input: &str) -> usize {
    let instructions = parse(input);

    let mut x = 0;
    let mut y = 0;
    let mut facing = Direction::East;

    let mut translate = |dir: Direction, value| {
        let (dx, dy) = dir.as_delta();
        x += dx * value;
        y += dy * value;
    };

    for instruction in instructions {
        match instruction.movement {
            Movement::Direction(direction) => {
                translate(direction, instruction.value);
            }
            Movement::Right => facing = facing.rotate(instruction.value),
            Movement::Left => facing = facing.rotate(-instruction.value),
            Movement::Forward => translate(facing, instruction.value),
        }
    }

    (x.abs() + y.abs()) as usize
}

fn rotate_delta(mut x: i32, mut y: i32, mut degrees: i32) -> (i32, i32) {
    while degrees < 0 {
        degrees = 360 + degrees;
    }

    for _ in 0..(degrees / 90) {
        // (x, y) = (y, -x);
        let tmp = x;
        x = y;
        y = -tmp;
    }

    (x, y)
}

pub fn part2(input: &str) -> usize {
    let instructions = parse(input);

    let mut x = 0;
    let mut y = 0;

    let mut wx = 10;
    let mut wy = 1;

    for instruction in instructions {
        match instruction.movement {
            Movement::Direction(direction) => {
                let (dx, dy) = direction.as_delta();
                wx += dx * instruction.value;
                wy += dy * instruction.value;
            }
            Movement::Right => {
                let (new_wx, new_wy) = rotate_delta(wx, wy, instruction.value);
                wx = new_wx;
                wy = new_wy;
            }
            Movement::Left => {
                let (new_wx, new_wy) = rotate_delta(wx, wy, -instruction.value);
                wx = new_wx;
                wy = new_wy;
            }
            Movement::Forward => {
                x += wx * instruction.value;
                y += wy * instruction.value;
            }
        }
    }

    (x.abs() + y.abs()) as usize
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, rotate_delta};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 25);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 286);
    }

    #[test]
    pub fn test_rotate_delta() {
        let (wx, wy) = (10, 1);
        assert_eq!(rotate_delta(wx, wy, 90), (1, -10));
        assert_eq!(rotate_delta(wx, wy, 180), (-10, -1));
        assert_eq!(rotate_delta(wx, wy, 270), (-1, 10));
        assert_eq!(rotate_delta(wx, wy, 360), (10, 1));

        assert_eq!(rotate_delta(wx, wy, -270), (1, -10));
        assert_eq!(rotate_delta(wx, wy, -180), (-10, -1));
        assert_eq!(rotate_delta(wx, wy, -90), (-1, 10));
        assert_eq!(rotate_delta(wx, wy, -450), (-1, 10));
        assert_eq!(rotate_delta(wx, wy, -360), (10, 1));
    }
}
