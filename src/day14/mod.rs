use std::{
    cmp::{max, min},
    collections::HashMap,
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn below(&self) -> impl Iterator<Item = Point> {
        [
            Point(self.0, self.1 + 1),
            Point(self.0 - 1, self.1 + 1),
            Point(self.0 + 1, self.1 + 1),
        ]
        .into_iter()
    }
}

impl FromStr for Point {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> crate::Result<Self> {
        let values: Vec<i32> = s.split(',').map(str::parse).collect::<Result<_, _>>()?;
        Ok(Point(values[0], values[1]))
    }
}

fn parse(input: &str) -> crate::Result<HashMap<Point, char>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let points: Vec<Point> = line
            .split(" -> ")
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        for points in points.windows(2) {
            let xmin = min(points[0].0, points[1].0);
            let xmax = max(points[0].0, points[1].0);
            let ymin = min(points[0].1, points[1].1);
            let ymax = max(points[0].1, points[1].1);
            for x in xmin..=xmax {
                for y in ymin..=ymax {
                    map.insert(Point(x, y), '#');
                }
            }
        }
    }
    Ok(map)
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let mut map = parse(input)?;
    let ymax = map.keys().map(|p| p.1).max().expect("map empty");
    let mut cnt = 0;
    loop {
        let mut p = Point(500, 0);
        while p.1 <= ymax {
            if let Some(pp) = p.below().find(|p| !map.contains_key(p)) {
                if pp.1 > ymax {
                    return Ok(cnt);
                }

                p = pp;
                continue;
            }

            map.insert(p, 'o');
            cnt += 1;
            break;
        }
    }
}

pub fn part2(input: &str) -> crate::Result<usize> {
    let mut map = parse(input)?;
    let floor = 2 + map.keys().map(|p| p.1).max().expect("map empty");
    let mut cnt = 0;
    loop {
        let mut p = Point(500, 0);
        loop {
            if p.1 == floor - 1 {
                map.insert(p, 'o');
                cnt += 1;
                break;
            }

            if let Some(pp) = p.below().find(|p| !map.contains_key(p)) {
                p = pp;
                continue;
            }

            map.insert(p, 'o');
            cnt += 1;
            if p == Point(500, 0) {
                return Ok(cnt);
            }
            break;
        }
    }
}
