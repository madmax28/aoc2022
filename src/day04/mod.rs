use std::str::FromStr;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

struct Range {
    from: i32,
    to: i32,
}

impl FromStr for Range {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split('-');
        Ok(Range {
            from: nums
                .next()
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
                .parse()?,
            to: nums
                .next()
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
                .parse()?,
        })
    }
}

struct Pair(Range, Range);

impl Pair {
    fn contained(&self) -> bool {
        self.0.from <= self.1.from && self.0.to >= self.1.to
            || self.1.from <= self.0.from && self.1.to >= self.0.to
    }

    fn overlaps(&self) -> bool {
        self.0.to >= self.1.from && self.0.from <= self.1.to
    }
}

impl FromStr for Pair {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs = s.split(',');
        Ok(Pair(
            pairs
                .next()
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
                .parse()?,
            pairs
                .next()
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
                .parse()?,
        ))
    }
}

pub fn part1(input: &str) -> crate::Result<usize> {
    input
        .lines()
        .map(str::parse::<Pair>)
        .try_fold(0, |acc, pair| {
            Ok(if pair?.contained() { 1 + acc } else { acc })
        })
}

pub fn part2(input: &str) -> crate::Result<usize> {
    input
        .lines()
        .map(str::parse::<Pair>)
        .try_fold(0, |acc, pair| {
            Ok(if pair?.overlaps() { 1 + acc } else { acc })
        })
}
