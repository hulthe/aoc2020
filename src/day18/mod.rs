#[derive(Clone, Copy, Debug)]
pub enum Token {
    Op(Op),
    Num(u64),
    LParen,
    RParen,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Mul,
}

/// A number, of an operator. Used for Reverse Polish Notation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RPN {
    Op(Op),
    Num(u64),
}

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = impl Iterator<Item = Token> + 'a> + 'a {
    input.lines().map(|line| {
        line.chars().filter_map(|c| {
            Some(match c {
                ')' => Token::RParen,
                '(' => Token::LParen,
                '*' => Token::Op(Op::Mul),
                '+' => Token::Op(Op::Add),
                '0' => Token::Num(0),
                '1' => Token::Num(1),
                '2' => Token::Num(2),
                '3' => Token::Num(3),
                '4' => Token::Num(4),
                '5' => Token::Num(5),
                '6' => Token::Num(6),
                '7' => Token::Num(7),
                '8' => Token::Num(8),
                '9' => Token::Num(9),
                ' ' => return None,
                _ => panic!("unexpected char: '{}'", c),
            })
        })
    })
}

/// Returns the input equation as reverse polish notation
///
/// The `has_higher_prescedence` function is used to determine if one operator binds more tightly
/// than another.
fn to_rpn<F>(input: &str, has_higher_prescedence: F) -> Vec<RPN>
where
    F: Fn(Op, Op) -> bool,
{
    #[derive(Clone, Copy, Debug)]
    enum StackElem {
        StackFrame,
        Op(Op),
    }

    /// An infinitive iterator where the first element is true, and the rest are false
    fn first_true() -> impl Iterator<Item = bool> {
        use std::iter;
        iter::once(true).chain(iter::repeat(false))
    }

    let mut stack = vec![];
    let mut output = vec![];

    for (line, is_first) in parse(input).zip(first_true()) {
        for token in line {
            match token {
                Token::Num(num) => output.push(RPN::Num(num)),
                Token::Op(op) => {
                    // make sure lhs is computed
                    match stack.last().copied() {
                        // stack is empty
                        None | Some(StackElem::StackFrame) => {}

                        // this operator had a higher prescedence than the previous one
                        // so we let the previous one wait
                        Some(StackElem::Op(prev_op)) if has_higher_prescedence(op, prev_op) => {}

                        // in any other case, we pop the previous operation from the stack
                        Some(StackElem::Op(prev_op)) => {
                            output.push(RPN::Op(prev_op));
                            stack.pop();
                        }
                    }

                    // then we push this operator to the stack and wait for rhs to be computed
                    stack.push(StackElem::Op(op));
                }
                Token::LParen => stack.push(StackElem::StackFrame),
                Token::RParen => loop {
                    // pop everything up until the last stackframe
                    match stack.pop() {
                        None | Some(StackElem::StackFrame) => break,
                        Some(StackElem::Op(op)) => output.push(RPN::Op(op)),
                    }
                },
            }
        }

        // pop everything from the stack
        while let Some(elem) = stack.pop() {
            match elem {
                StackElem::Op(op) => output.push(RPN::Op(op)),
                StackElem::StackFrame => {}
            }
        }

        if !is_first {
            // in order to sum all the lines, we inject an addition here
            output.push(RPN::Op(Op::Add));
        }
    }

    output
}

fn evalutate_rpn(ops: &[RPN]) -> u64 {
    let mut stack = Vec::new();

    for &op in ops {
        match op {
            RPN::Num(num) => stack.push(num),
            RPN::Op(op) => {
                let rhs = stack.pop().expect("invalid op arg count, was 0");
                let lhs = stack.pop().expect("invalid op arg count, was 1");

                let result = match op {
                    Op::Add => lhs + rhs,
                    Op::Mul => lhs * rhs,
                };

                stack.push(result);
            }
        }
    }

    debug_assert_eq!(
        stack.len(),
        1,
        "invalid input, stack len was {} instead of 1",
        stack.len()
    );

    stack.pop().unwrap()
}

pub fn part1(input: &str) -> u64 {
    let rpn = to_rpn(
        input,
        |_, _| false, /* no operator has a higher prescedence */
    );
    evalutate_rpn(&rpn)
}

pub fn part2(input: &str) -> u64 {
    let rpn = to_rpn(
        input,
        |op1, op2| (op1, op2) == (Op::Add, Op::Mul), // + binds more tightly than *
    );
    evalutate_rpn(&rpn)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 26335);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 693891);
    }
}
