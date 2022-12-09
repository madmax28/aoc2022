use std::collections::HashSet;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

type Point = (i32, i32);

struct Rope {
    knots: Vec<Point>,
    visited: HashSet<Point>,
}

impl Rope {
    fn new(len: usize) -> Self {
        let mut visited = HashSet::new();
        visited.insert((0, 0));

        Rope {
            knots: vec![(0, 0); len],
            visited,
        }
    }

    fn follow(&mut self, idx: usize) {
        let dx = self.knots[idx - 1].0 - self.knots[idx].0;
        let dy = self.knots[idx - 1].1 - self.knots[idx].1;

        if dx.abs() > 1 || dy.abs() > 1 {
            if dx != 0 {
                self.knots[idx].0 += if dx > 0 { 1 } else { -1 };
            }
            if dy != 0 {
                self.knots[idx].1 += if dy > 0 { 1 } else { -1 };
            }
        }
    }

    fn mv(&mut self, dir: Point, cnt: i32) {
        for _ in 0..cnt {
            self.knots[0].0 += dir.0;
            self.knots[0].1 += dir.1;

            for idx in 1..self.knots.len() {
                self.follow(idx);
            }

            self.visited.insert(*self.knots.last().unwrap());
        }
    }
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let mut rope = Rope::new(2);
    for line in input.lines() {
        let cnt = line[2..].parse::<i32>()?;
        match &line[..1] {
            "R" => rope.mv((1, 0), cnt),
            "L" => rope.mv((-1, 0), cnt),
            "D" => rope.mv((0, 1), cnt),
            "U" => rope.mv((0, -1), cnt),
            _ => return Err(crate::Error::boxed(Error::InvalidInput)),
        }
    }
    Ok(rope.visited.len())
}

pub fn part2(input: &str) -> crate::Result<usize> {
    let mut rope = Rope::new(10);
    for line in input.lines() {
        let cnt = line[2..].parse::<i32>()?;
        match &line[..1] {
            "R" => rope.mv((1, 0), cnt),
            "L" => rope.mv((-1, 0), cnt),
            "D" => rope.mv((0, 1), cnt),
            "U" => rope.mv((0, -1), cnt),
            _ => return Err(crate::Error::boxed(Error::InvalidInput)),
        }
    }
    Ok(rope.visited.len())
}
