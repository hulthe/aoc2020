use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

pub type Range = RangeInclusive<u64>;

#[derive(Debug)]
pub struct FieldRanges<'a> {
    inner: HashMap<&'a str, (Range, Range)>,
}

#[derive(Debug)]
pub struct Input<'a> {
    field_ranges: FieldRanges<'a>,
    nearby_tickets: Vec<Vec<u64>>,
    my_ticket: Vec<u64>,
}

pub fn parse(input: &str) -> Input {
    let mut sections = input.split("\n\n");
    let field_ranges_input = sections.next().unwrap();
    let my_ticket_input = sections.next().unwrap();
    let nearby_tickets_input = sections.next().unwrap();

    // oh holy lords of parsing, please have mercy on my soul

    fn to_range(input: &str) -> Range {
        let (from, to) = input.split_once("-").unwrap();
        let from = from.parse().unwrap();
        let to = to.parse().unwrap();
        Range::new(from, to)
    }

    let field_ranges = field_ranges_input
        .lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(name, ranges)| {
            ranges
                .split_once(" or ")
                .map(|(r1, r2)| (name, (to_range(r1), to_range(r2))))
        })
        .collect();

    let my_ticket = my_ticket_input
        .lines()
        .skip(1)
        .next()
        .iter()
        .flat_map(|line| line.split(','))
        .map(|field| field.parse().unwrap())
        .collect();

    let nearby_tickets = nearby_tickets_input
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|field| field.parse().unwrap())
                .collect()
        })
        .collect();

    Input {
        field_ranges: FieldRanges {
            inner: field_ranges,
        },
        my_ticket,
        nearby_tickets,
    }
}

impl FieldRanges<'_> {
    /// Return an iterator over which fields the number is valid for
    pub fn number_valid_for(&self, number: u64) -> impl Iterator<Item = &str> {
        self.inner
            .iter()
            .filter(move |(_, (r1, r2))| r1.contains(&number) || r2.contains(&number))
            .map(|(&name, _)| name)
    }
}

pub fn part1(input: &str) -> u64 {
    let input = parse(input);

    input
        .nearby_tickets
        .iter()
        .flat_map(|fields| fields.iter())
        .map(|&field| field)
        .filter(|&field| input.field_ranges.number_valid_for(field).next().is_none())
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let Input {
        field_ranges,
        mut nearby_tickets,
        my_ticket,
    } = parse(input);

    // discard tickets with invalid fields
    nearby_tickets.retain(|ticket| {
        ticket
            .iter()
            .map(|&field| field_ranges.number_valid_for(field).next().is_some())
            .all(|retain| retain)
    });

    let mut field_possibilities: Vec<HashSet<&str>> = my_ticket
        .iter()
        .map(|_| field_ranges.inner.keys().copied().collect())
        .collect();

    for ticket in nearby_tickets {
        for (i, &field) in ticket.iter().enumerate() {
            let valid_for: HashSet<&str> = field_ranges.number_valid_for(field).collect();
            field_possibilities[i].retain(|field| valid_for.contains(field));
        }
    }

    let mut field_names: Vec<Option<&str>> = vec![None; field_possibilities.len()];
    loop {
        // find next field with only one possible match
        let (i, next_set) = field_possibilities
            .iter_mut()
            .enumerate()
            .filter(|(_, set)| set.len() == 1)
            .next()
            .unwrap();

        let next = next_set.drain().next().unwrap();

        for set in field_possibilities.iter_mut() {
            set.remove(next);
        }

        field_names[i] = Some(next);

        if field_names.iter().all(|name| name.is_some()) {
            break;
        }
    }

    field_names
        .iter()
        .flatten()
        .enumerate()
        .filter(|(_, name)| name.starts_with("departure"))
        .map(|(i, _name)| my_ticket[i])
        .product()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 71);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 1337);
    }
}
