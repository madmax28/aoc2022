use std::collections::HashSet;

fn priority(c: char) -> usize {
    if c.is_ascii_lowercase() {
        c as usize - 96
    } else {
        c as usize - 64 + 26
    }
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let mut res = 0;
    for line in input.lines() {
        let compartment: HashSet<_> = line.chars().take(line.len() / 2).collect();
        res += line
            .chars()
            .skip(line.len() / 2)
            .find(|c| compartment.contains(c))
            .map(priority)
            .expect("invalid input");
    }
    Ok(res)
}

pub fn part2(input: &str) -> crate::Result<usize> {
    Ok(input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            group
                .iter()
                .map(|elf| elf.chars().collect::<HashSet<_>>())
                .reduce(|a, b| a.intersection(&b).copied().collect())
                .map(|set| set.into_iter().next().expect("invalid input"))
                .map(priority)
                .expect("invalid input")
        })
        .sum())
}
