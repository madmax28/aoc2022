use std::cmp::min;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

fn stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in input.lines().take_while(|line| line.contains('[')) {
        for (idx, start) in (0..line.len()).step_by(4).enumerate() {
            if idx >= stacks.len() {
                stacks.push(Vec::new());
            }

            let end = min(line.len(), start + 4);
            if let Some(c) = line[start..end].chars().find(char::is_ascii_alphabetic) {
                stacks[idx].push(c);
            }
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    stacks
}

fn perform(input: &str, mut stacks: Vec<Vec<char>>, is_9001: bool) -> crate::Result<String> {
    for line in input.lines().skip_while(|line| !line.starts_with("move")) {
        let mut crates = line
            .split(' ')
            .filter_map(|token| token.parse::<usize>().ok());
        let count = crates
            .next()
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
        let from = crates
            .next()
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
            - 1;
        let to = crates
            .next()
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
            - 1;

        let end = stacks[from].len() - count;
        let mut crates = stacks[from].split_off(end);
        if !is_9001 {
            crates.reverse();
        }
        stacks[to].extend(crates);
    }

    Ok(stacks
        .into_iter()
        .map(|mut stack| {
            stack
                .pop()
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))
        })
        .collect::<Result<_, _>>()?)
}

pub fn part1(input: &str) -> crate::Result<String> {
    let stacks = stacks(input);
    perform(input, stacks, false)
}

pub fn part2(input: &str) -> crate::Result<String> {
    let stacks = stacks(input);
    perform(input, stacks, true)
}
