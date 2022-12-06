use std::collections::HashSet;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

pub fn part1(input: &str) -> crate::Result<usize> {
    for idx in 0..input.len() - 4 {
        let set: HashSet<char> = input[idx..idx + 4].chars().collect();
        if set.len() == 4 {
            return Ok(idx + 4);
        }
    }
    Err(crate::Error::boxed(Error::InvalidInput))
}

pub fn part2(input: &str) -> crate::Result<usize> {
    for idx in 0..input.len() - 14 {
        let set: HashSet<char> = input[idx..idx + 14].chars().collect();
        if set.len() == 14 {
            return Ok(idx + 14);
        }
    }
    Err(crate::Error::boxed(Error::InvalidInput))
}
