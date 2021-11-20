pub struct Data {
    /// earliest timestamp you could depart on a bus
    earliest: u64,

    busses: Vec<Bus>,
}

#[derive(Clone, Copy)]
pub struct Bus {
    id: u64,
    offset: u64,
}

pub fn parse(input: &str) -> Data {
    let mut offset = 0;
    Data {
        earliest: input.lines().next().unwrap().parse().unwrap(),
        busses: input
            .lines()
            .skip(1)
            .next()
            .unwrap()
            .split(",")
            .filter_map(|line| {
                let r = match line {
                    "x" => None,
                    num => Some(Bus {
                        offset,
                        id: num.parse().unwrap(),
                    }),
                };
                offset += 1;
                r
            })
            .collect(),
    }
}

pub fn part1(input: &str) -> u64 {
    let data = parse(input);
    data.busses
        .iter()
        .map(|bus| bus.id)
        .map(|id| {
            let next_departure = id - (data.earliest % id);
            (next_departure, id)
        })
        .min()
        .map(|(wait, id)| wait * id)
        .expect("failed to find a solution")
}

pub fn part2(input: &str) -> u64 {
    let data = parse(input);

    // solve chinese remainder theorem
    // method shamelessly stolen from online math course

    fn solve_xi(n: u64, m: u64) -> u64 {
        let mut x = 1;
        let mut nx = n;
        while nx % m != 1 {
            x += 1;
            nx += n;
        }
        return x;
    }

    let n: u64 = data.busses.iter().map(|bus| bus.id).product();

    let mut bnx = 0;
    for bus in data.busses.into_iter().skip(1) {
        let i = bus.offset;
        let ni = n / bus.id;
        let xi = solve_xi(ni, bus.id);
        bnx += i * ni * xi;
    }

    n - (bnx % n)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 295);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 1068781);
    }
}
