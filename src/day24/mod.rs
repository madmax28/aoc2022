use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    iter,
};

#[derive(Debug, Clone)]
enum Tile {
    Wall,
    Space(Vec<char>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point(i32, i32);

impl Point {
    fn neighbors(&self) -> impl Iterator<Item = Point> {
        [
            Point(self.0 + 1, self.1),
            Point(self.0 - 1, self.1),
            Point(self.0, self.1 + 1),
            Point(self.0, self.1 - 1),
        ]
        .into_iter()
    }

    fn mv(&self, dir: char) -> Self {
        match dir {
            '>' => Point(self.0 + 1, self.1),
            '<' => Point(self.0 - 1, self.1),
            'v' => Point(self.0, self.1 + 1),
            '^' => Point(self.0, self.1 - 1),
            _ => panic!("invalid dir"),
        }
    }

    fn dist(&self, other: Point) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

#[derive(Debug, Clone)]
struct Map(Vec<Vec<Tile>>);

impl Map {
    fn height(&self) -> i32 {
        self.0.len() as i32
    }

    fn width(&self) -> i32 {
        self.0[0].len() as i32
    }

    fn put_blizzard(&mut self, p: Point, c: char) {
        if let Tile::Space(blizzards) = &mut self.0[p.1 as usize][p.0 as usize] {
            blizzards.push(c);
        } else {
            panic!("can't put blizzard on wall tile");
        }
    }

    fn put_wall(&mut self, p: Point) {
        self.0[p.1 as usize][p.0 as usize] = Tile::Wall;
    }

    fn get(&self, p: Point) -> &Tile {
        &self.0[p.1 as usize][p.0 as usize]
    }

    fn wrap(&self, p: Point, dir: char) -> Point {
        match dir {
            '>' => Point(1, p.1),
            '<' => Point(self.width() - 2, p.1),
            'v' => Point(p.0, 1),
            '^' => Point(p.0, self.height() - 2),
            _ => panic!("invalid dir"),
        }
    }

    fn neighbors(&self, p: Point) -> impl Iterator<Item = Point> {
        let h = self.height();
        let w = self.width();
        p.neighbors()
            .filter(move |p| p.0 >= 0 && p.0 < w && p.1 >= 0 && p.1 < h)
    }

    fn tick(&self) -> Self {
        let mut new = Map(vec![
            vec![Tile::Space(Vec::new()); self.width() as usize];
            self.height() as usize
        ]);

        for point in (0..self.width())
            .flat_map(|y| iter::repeat(y).zip(0..self.height()))
            .map(|(x, y)| Point(x as i32, y as i32))
        {
            match self.get(point) {
                Tile::Wall => new.put_wall(point),
                Tile::Space(blizzards) => {
                    for blizzard in blizzards {
                        let mut cand = point.mv(*blizzard);
                        if let Tile::Wall = self.get(cand) {
                            cand = self.wrap(cand, *blizzard);
                        }
                        new.put_blizzard(cand, *blizzard);
                    }
                }
            }
        }
        new
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!();
        for y in 0..self.height() {
            for x in 0..self.width() {
                match &self.0[y as usize][x as usize] {
                    Tile::Wall => print!("#"),
                    Tile::Space(blizzards) => {
                        if blizzards.is_empty() {
                            print!(".");
                        } else if blizzards.len() == 1 {
                            print!("{}", blizzards[0]);
                        } else {
                            print!("M");
                        }
                    }
                }
            }
            println!();
        }
    }
}

fn a_star(start: Point, goal: Point, minutes: i32, maps: &[Map]) -> Option<i32> {
    let mut frontier = BinaryHeap::new();
    let heuristic = minutes + start.dist(goal);
    frontier.push(Reverse((heuristic, minutes, start)));

    let mut visited = HashSet::new();
    while let Some(Reverse((_, mut minutes, pos))) = frontier.pop() {
        if !visited.insert((minutes, pos)) {
            continue;
        }

        minutes += 1;
        let map = &maps[minutes as usize % maps.len()];

        for new_pos in map.neighbors(pos) {
            if let Tile::Space(blizzards) = map.get(new_pos) {
                if blizzards.is_empty() {
                    if new_pos == goal {
                        return Some(minutes);
                    }

                    let heuristic = minutes + new_pos.dist(goal);
                    frontier.push(Reverse((heuristic, minutes, new_pos)));
                }
            }
        }

        if let Tile::Space(blizzards) = map.get(pos) {
            if blizzards.is_empty() {
                let heuristic = minutes + pos.dist(goal);
                frontier.push(Reverse((heuristic, minutes, pos)));
            }
        }
    }
    None
}

fn parse(input: &str) -> (Map, Point, Point) {
    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            match c {
                '#' => row.push(Tile::Wall),
                '.' => row.push(Tile::Space(Vec::new())),
                c => row.push(Tile::Space(vec![c])),
            }
        }
        map.push(row);
    }

    let (start, goal) = (
        Point(1, 0),
        Point(map[0].len() as i32 - 2, map.len() as i32 - 1),
    );

    (Map(map), start, goal)
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let (map, start, goal) = parse(input);

    let (w, h) = (map.width() - 2, map.height() - 2);
    let lcm = w * h / gcd(w, h);
    let mut maps = vec![map];
    for _ in 0..lcm - 1 {
        maps.push(maps.last().unwrap().tick());
    }

    Ok(a_star(start, goal, 0, &maps).expect("no path found"))
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let (map, start, goal) = parse(input);

    let (w, h) = (map.width() - 2, map.height() - 2);
    let lcm = w * h / gcd(w, h);
    let mut maps = vec![map];
    for _ in 0..lcm - 1 {
        maps.push(maps.last().unwrap().tick());
    }

    let minutes = a_star(start, goal, 0, &maps).expect("no path found");
    let minutes = a_star(goal, start, minutes, &maps).expect("no path found");
    let minutes = a_star(start, goal, minutes, &maps).expect("no path found");
    Ok(minutes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

        assert_eq!(part1(input).unwrap(), 18);
        assert_eq!(part2(input).unwrap(), 54);
    }
}
