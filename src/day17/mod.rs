use std::collections::HashSet;

pub type XY = [i32; 2];
pub type XYZ = [i32; 3];
pub type XYZW = [i32; 4];

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = XY> + 'a {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ([x as i32, y as i32], c))
        })
        .filter(|&(_, c)| c == '#')
        .map(|(coord, _)| coord)
}

pub fn part1(input: &str) -> usize {
    let mut active: HashSet<XYZ> = parse(input).map(|[x, y]| [x, y, 0]).collect();

    for _cycle in 0..6 {
        // create an iterator over inactive spaces that will *become* active
        let new_active = active
            .iter()
            .copied()
            // all neighbors of active cubes
            .flat_map(|coord| neighbors3(coord))
            // ...that are non-active
            .filter(|neighbor| !active.contains(neighbor))
            // ...with exactly 3 active neighbors
            .filter(|&inactive| {
                neighbors3(inactive)
                    .filter(|neighbor| active.contains(neighbor))
                    .count()
                    == 3
            });

        // create an iterator over active spaces that will *remain* active
        let old_active = active.iter().copied().filter(|&coord| {
            [2, 3].contains(
                &neighbors3(coord)
                    .filter(|neighbor| active.contains(neighbor))
                    .count(),
            )
        });

        // create the new set of active cubes
        active = new_active.chain(old_active).collect();
    }

    active.len()
}

pub fn part2(input: &str) -> usize {
    let mut active: HashSet<XYZW> = parse(input).map(|[x, y]| [x, y, 0, 0]).collect();

    for _cycle in 0..6 {
        // create an iterator over inactive spaces that will *become* active
        let new_active = active
            .iter()
            .copied()
            // all neighbors of active cubes
            .flat_map(|coord| neighbors4(coord))
            // ...that are non-active
            .filter(|neighbor| !active.contains(neighbor))
            // ...with exactly 3 active neighbors
            .filter(|&inactive| {
                neighbors4(inactive)
                    .filter(|neighbor| active.contains(neighbor))
                    .count()
                    == 3
            });

        // create an iterator over active spaces that will *remain* active
        let old_active = active.iter().copied().filter(|&coord| {
            [2, 3].contains(
                &neighbors4(coord)
                    .filter(|neighbor| active.contains(neighbor))
                    .count(),
            )
        });

        // create the new set of active cubes
        active = new_active.chain(old_active).collect();
    }

    active.len()
}

fn neighbors3([x, y, z]: XYZ) -> impl Iterator<Item = XYZ> {
    (-1..=1)
        .flat_map(|x| (-1..=1).map(move |y| [x, y]))
        .flat_map(|[x, y]| (-1..=1).map(move |z| [x, y, z]))
        .filter(move |&n| n != [0; 3])
        .map(move |[nx, ny, nz]| [x + nx, y + ny, z + nz])
}

fn neighbors4([x, y, z, w]: XYZW) -> impl Iterator<Item = XYZW> {
    (-1..=1)
        .flat_map(|x| (-1..=1).map(move |y| [x, y]))
        .flat_map(|[x, y]| (-1..=1).map(move |z| [x, y, z]))
        .flat_map(|[x, y, z]| (-1..=1).map(move |w| [x, y, z, w]))
        .filter(move |&n| n != [0; 4])
        .map(move |[nx, ny, nz, nw]| [x + nx, y + ny, z + nz, w + nw])
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 112);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 848);
    }
}
