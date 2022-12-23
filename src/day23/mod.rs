use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    iter,
};

fn neighbors(pos: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    [
        (pos.0 - 1, pos.1 - 1),
        (pos.0 - 1, pos.1),
        (pos.0 - 1, pos.1 + 1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
        (pos.0 + 1, pos.1 - 1),
        (pos.0 + 1, pos.1),
        (pos.0 + 1, pos.1 + 1),
    ]
    .into_iter()
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn step(self, from: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::North => (from.0, from.1 - 1),
            Direction::South => (from.0, from.1 + 1),
            Direction::West => (from.0 - 1, from.1),
            Direction::East => (from.0 + 1, from.1),
        }
    }

    fn look(self, from: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
        neighbors(from).filter(move |(x, y)| match self {
            Direction::North => *y == from.1 - 1,
            Direction::South => *y == from.1 + 1,
            Direction::West => *x == from.0 - 1,
            Direction::East => *x == from.0 + 1,
        })
    }
}

#[derive(Debug)]
struct Elf {
    pos: (i32, i32),
    proposal: Option<(i32, i32)>,
}

impl Elf {
    fn new(pos: (i32, i32)) -> Self {
        Elf {
            pos,
            proposal: None,
        }
    }

    fn propose(&mut self, positions: &HashSet<(i32, i32)>, directions: &[Direction]) {
        self.proposal = None;

        if neighbors(self.pos).all(|p| !positions.contains(&p)) {
            return;
        }

        for dir in directions {
            if dir.look(self.pos).all(|p| !positions.contains(&p)) {
                self.proposal = Some(dir.step(self.pos));
                return;
            }
        }
    }

    fn mv(&mut self, proposals: &HashMap<(i32, i32), i32>) -> bool {
        if let Some(pos) = self.proposal {
            if proposals[&pos] < 2 {
                self.pos = pos;
                return true;
            }
        }
        false
    }
}

fn parse(s: &str) -> (Vec<Elf>, HashSet<(i32, i32)>) {
    let mut elves = Vec::new();
    let mut positions = HashSet::new();
    for (y, (x, _)) in s
        .lines()
        .enumerate()
        .flat_map(|(y, line)| iter::repeat(y).zip(line.chars().enumerate()))
        .filter(|(_, (_, c))| *c == '#')
    {
        elves.push(Elf::new((x as i32, y as i32)));
        positions.insert((x as i32, y as i32));
    }
    (elves, positions)
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let (mut elves, mut positions) = parse(input);
    let mut directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    for _ in 0..10 {
        for elf in &mut elves {
            elf.propose(&positions, &directions);
        }

        let mut proposed = HashMap::new();
        for pos in elves.iter().filter_map(|e| e.proposal) {
            proposed.entry(pos).and_modify(|v| *v += 1).or_insert(1);
        }

        positions.clear();
        for elf in &mut elves {
            elf.mv(&proposed);
            positions.insert(elf.pos);
        }

        directions.rotate_left(1);
    }

    let from = elves
        .iter()
        .map(|elf| elf.pos)
        .reduce(|p1, p2| (min(p1.0, p2.0), min(p1.1, p2.1)))
        .unwrap();
    let to = elves
        .iter()
        .map(|elf| elf.pos)
        .reduce(|p1, p2| (max(p1.0, p2.0), max(p1.1, p2.1)))
        .unwrap();

    let covered = (1 + to.0 - from.0) * (1 + to.1 - from.1) - elves.len() as i32;
    Ok(covered)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let (mut elves, mut positions) = parse(input);
    let mut directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    for round in 1.. {
        for elf in &mut elves {
            elf.propose(&positions, &directions);
        }

        let mut proposed = HashMap::new();
        for pos in elves.iter().filter_map(|e| e.proposal) {
            proposed.entry(pos).and_modify(|v| *v += 1).or_insert(1);
        }

        positions.clear();
        let mut moved = false;
        for elf in &mut elves {
            if elf.mv(&proposed) {
                moved = true;
            }
            positions.insert(elf.pos);
        }

        if !moved {
            return Ok(round);
        }

        directions.rotate_left(1);
    }
    unreachable!()
}
