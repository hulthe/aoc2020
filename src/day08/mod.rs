#[derive(Clone, Copy, Debug)]
enum InstrKind {
    Acc,
    Jmp,
    Nop,
}

#[derive(Clone, Copy, Debug)]
struct Instr {
    kind: InstrKind,
    arg: i32,
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item = Instr> + 'a {
    input
        .lines()
        .flat_map(|line| line.split_once(' '))
        .map(|(instr_kind, arg)| {
            let kind = match instr_kind {
                "nop" => InstrKind::Nop,
                "acc" => InstrKind::Acc,
                "jmp" => InstrKind::Jmp,
                _ => panic!("unknown instruction: {}", instr_kind),
            };

            Instr {
                kind,
                arg: arg.parse().unwrap(),
            }
        })
}

/// Run the instruction set
///
/// Returns false if a loop was detected, true otherwise
fn run(instrs: &mut [(Instr, bool)], acc: &mut i32) -> bool {
    let mut pc = 0;

    loop {
        if pc >= instrs.len() {
            return true;
        }

        let (instr, visited) = &mut instrs[pc];

        if *visited {
            return false;
        }

        *visited = true;

        match instr.kind {
            InstrKind::Nop => {}
            InstrKind::Acc => {
                *acc += instr.arg;
            }
            InstrKind::Jmp => {
                pc = ((pc as i32) + instr.arg) as usize;
                continue;
            }
        }
        pc += 1;
    }
}

pub fn part1(input: &str) -> i32 {
    let mut instrs: Vec<_> = parse(input).map(|instr| (instr, false)).collect();
    let mut acc = 0;

    run(&mut instrs, &mut acc);

    return acc;
}

pub fn part2(input: &str) -> i32 {
    let mut instrs: Vec<_> = parse(input).map(|instr| (instr, false)).collect();
    let mut acc = 0;

    // For every instruction, try replacing Nop with Jmp and vice versa to fix the program
    for i in (0..instrs.len()).rev() {
        let prev: InstrKind = instrs[i].0.kind;

        // Try replacing instruction
        instrs[i].0.kind = match instrs[i].0.kind {
            InstrKind::Jmp => InstrKind::Nop,
            InstrKind::Nop => InstrKind::Jmp,
            kind => kind,
        };

        // Check if that fixed it
        if run(&mut instrs, &mut acc) {
            return acc;
        }

        // It didn't work, restore instruction to what it was before
        instrs[i].0.kind = prev;

        // Reset accumulator and visited
        acc = 0;
        instrs.iter_mut().for_each(|(_, visited)| *visited = false);
    }

    panic!("no solution found");
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 5);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 8);
    }
}
