use std::{collections::VecDeque, str::FromStr};

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug)]
enum Op {
    Old,
    Num(u64),
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: (Op, char, Op),
    test: (u64, usize, usize),
    throw_count: u64,
}

impl Monkey {
    fn inspect<F>(&mut self, op: F) -> crate::Result<Option<(u64, usize)>>
    where
        F: Fn(u64, char, u64) -> crate::Result<u64>,
    {
        let worry = {
            let old = match self.items.pop_front() {
                Some(old) => old,
                None => return Ok(None),
            };
            let op1 = match self.operation.0 {
                Op::Old => old,
                Op::Num(n) => n,
            };
            let op2 = match self.operation.2 {
                Op::Old => old,
                Op::Num(n) => n,
            };
            op(op1, self.operation.1, op2)?
        };

        let to = if worry % self.test.0 == 0 {
            self.test.1
        } else {
            self.test.2
        };

        self.throw_count += 1;
        Ok(Some((worry, to)))
    }
}

impl FromStr for Monkey {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let items = lines[1][18..]
            .split(", ")
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        let operation = {
            let tokens: Vec<_> = lines[2][19..].split(' ').collect();
            let op1 = if tokens[0] == "old" {
                Op::Old
            } else {
                Op::Num(tokens[0].parse()?)
            };
            let op = tokens[1]
                .chars()
                .next()
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
            let op2 = if tokens[2] == "old" {
                Op::Old
            } else {
                Op::Num(tokens[2].parse()?)
            };
            (op1, op, op2)
        };
        let test = (
            lines[3][21..].parse()?,
            lines[4][29..].parse()?,
            lines[5][30..].parse()?,
        );
        Ok(Monkey {
            items,
            operation,
            test,
            throw_count: 0,
        })
    }
}

fn monkey_business<F>(mut monkeys: Vec<Monkey>, rounds: u64, op: F) -> crate::Result<u64>
where
    F: Fn(u64, char, u64) -> crate::Result<u64>,
{
    for _ in 0..rounds {
        for idx in 0..monkeys.len() {
            while let Some((worry, to)) = monkeys[idx].inspect(&op)? {
                monkeys[to].items.push_back(worry);
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.throw_count);
    let monkey_business = monkeys
        .into_iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.throw_count)
        .product();
    Ok(monkey_business)
}

pub fn part1(input: &str) -> crate::Result<u64> {
    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    monkey_business(monkeys, 20, |op1, op, op2| match op {
        '*' => Ok((op1 * op2) / 3),
        '+' => Ok((op1 + op2) / 3),
        _ => Err(crate::Error::boxed(Error::InvalidInput)),
    })
}

pub fn part2(input: &str) -> crate::Result<u64> {
    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    let divisor: u64 = monkeys.iter().map(|monkey| monkey.test.0).product();
    monkey_business(monkeys, 10_000, |op1, op, op2| match op {
        '*' => Ok((op1 * op2) % divisor),
        '+' => Ok((op1 + op2) % divisor),
        _ => Err(crate::Error::boxed(Error::InvalidInput)),
    })
}
