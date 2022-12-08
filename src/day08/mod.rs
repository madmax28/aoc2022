use std::{collections::HashMap, iter, str::FromStr};

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: &[Direction] = &[
    Direction::Right,
    Direction::Left,
    Direction::Down,
    Direction::Up,
];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point(i32, i32);

impl Point {
    fn neighbors(&self, dir: Direction) -> impl Iterator<Item = Point> {
        let mut needle = *self;
        iter::from_fn(move || {
            match dir {
                Direction::Right => needle.0 += 1,
                Direction::Left => needle.0 -= 1,
                Direction::Down => needle.1 += 1,
                Direction::Up => needle.1 -= 1,
            }
            Some(needle)
        })
    }
}

struct Map {
    trees: HashMap<Point, u32>,
}

impl FromStr for Map {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> crate::Result<Self> {
        let mut trees = HashMap::new();
        for (y, xs) in s.lines().enumerate() {
            for (x, c) in xs.chars().enumerate() {
                trees.insert(
                    Point(x as i32, y as i32),
                    c.to_digit(10)
                        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?,
                );
            }
        }

        Ok(Map { trees })
    }
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let map: Map = input.parse()?;
    let num_visible = map
        .trees
        .keys()
        .map(|pos| {
            DIRECTIONS
                .iter()
                .map(|dir| {
                    pos.neighbors(*dir)
                        .map(|pos| map.trees.get(&pos))
                        .take_while(Option::is_some)
                        .map(Option::unwrap)
                        .all(|other| other < &map.trees[pos])
                })
                .any(|visible| visible)
        })
        .filter(|visible| *visible)
        .count();
    Ok(num_visible)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let map: Map = input.parse()?;
    let score = map
        .trees
        .keys()
        .map(|pos| {
            DIRECTIONS
                .iter()
                .map(|dir| {
                    let mut score = 0;
                    for other in pos
                        .neighbors(*dir)
                        .map(|pos| map.trees.get(&pos))
                        .take_while(Option::is_some)
                        .map(Option::unwrap)
                    {
                        score += 1;
                        if other >= &map.trees[pos] {
                            break;
                        }
                    }
                    score
                })
                .product::<i32>()
        })
        .max()
        .unwrap_or(0);
    Ok(score)
}
