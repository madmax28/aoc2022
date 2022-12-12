use std::{
    cmp::{max, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

const A: i32 = b'a' as i32;
const Z: i32 = b'z' as i32;

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

    fn dist(&self, other: Point) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

fn a_star(start: Point, goal: Point, map: &HashMap<Point, i32>) -> Option<i32> {
    let mut frontier = BinaryHeap::new();
    let heuristic = max(start.dist(goal), Z - A);
    frontier.push(Reverse((heuristic, 0, start)));
    let mut visited = HashSet::new();

    while let Some(Reverse((_, cost, p))) = frontier.pop() {
        visited.insert(p);

        for np in p.neighbors() {
            if frontier.iter().any(|Reverse((_, _, p))| *p == np) {
                continue;
            }

            if let Some(height) = map.get(&np) {
                if visited.contains(&np) {
                    continue;
                }

                if (*height - map[&p]) > 1 {
                    continue;
                }

                if np == goal {
                    return Some(cost + 1);
                }

                let cost = 1 + cost;
                let heuristic = cost + max(np.dist(goal), Z - *height);
                frontier.push(Reverse((heuristic, cost, np)));
            }
        }
    }
    None
}

fn parse(input: &str) -> (HashMap<Point, i32>, Point, Point) {
    let mut map = HashMap::new();
    let (mut start, mut goal) = (Point(0, 0), Point(0, 0));
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            if c == 'S' {
                start = Point(x, y);
                map.insert(Point(x, y), A);
            } else if c == 'E' {
                goal = Point(x, y);
                map.insert(Point(x, y), Z);
            } else {
                map.insert(Point(x, y), c as i32);
            }
        }
    }
    (map, start, goal)
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let (map, start, goal) = parse(input);
    Ok(a_star(start, goal, &map).expect("no path found"))
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let (map, _, goal) = parse(input);
    let min = map
        .iter()
        .filter(|(_, &h)| h == A)
        .filter_map(|(p, _)| a_star(*p, goal, &map))
        .min()
        .unwrap();
    Ok(min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        assert_eq!(part1(input).unwrap(), 31);
        assert_eq!(part2(input).unwrap(), 29);
    }
}
