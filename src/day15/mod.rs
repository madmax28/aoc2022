use std::{
    cmp::{max, min},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point(i64, i64);

impl Point {
    fn dist(&self, other: Point) -> i64 {
        max(self.0, other.0) - min(self.0, other.0) + max(self.1, other.1) - min(self.1, other.1)
    }
}

#[derive(Debug, Clone, Copy)]
struct Range(i64, i64);

impl Range {
    fn contains(&self, n: i64) -> bool {
        n >= self.0 && n <= self.1
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Point,
    beacon: Point,
    range: i64,
}

impl Sensor {
    fn xrange(&self, y: i64) -> Option<Range> {
        let perpendicular_dist = self.pos.dist(Point(self.pos.0, y));
        if perpendicular_dist <= self.range {
            Some(Range(
                self.pos.0 - (self.range - perpendicular_dist),
                self.pos.0 + (self.range - perpendicular_dist),
            ))
        } else {
            None
        }
    }

    fn in_range(&self, pos: Point) -> bool {
        pos.dist(self.pos) <= self.range
    }

    fn candidates(&self) -> impl Iterator<Item = Point> {
        let xmin = self.pos.0 - self.range - 1;
        let xmax = self.pos.0 + self.range + 1;
        let ymin = self.pos.1 - self.range - 1;
        let ymax = self.pos.1 + self.range + 1;

        let xs = (xmin..=xmax).rev().chain(xmin..=xmax);
        let ys = (ymin..=self.pos.1)
            .rev()
            .chain(ymin..=ymax)
            .chain((self.pos.1..=ymax).rev());

        xs.zip(ys).map(|(x, y)| Point(x, y))
    }
}

impl FromStr for Sensor {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> crate::Result<Self> {
        let num = |s: &str| -> crate::Result<i64> {
            Ok(s.chars()
                .skip_while(|c| *c != '-' && !c.is_ascii_digit())
                .take_while(|c| *c == '-' || c.is_ascii_digit())
                .collect::<String>()
                .parse()?)
        };

        let nums: Vec<i64> = s
            .split(": ")
            .flat_map(|s| s.split(", "))
            .map(num)
            .collect::<Result<_, _>>()?;
        let pos = Point(nums[0], nums[1]);
        let beacon = Point(nums[2], nums[3]);
        let range = pos.dist(beacon);

        Ok(Sensor { pos, beacon, range })
    }
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let sensors: Vec<Sensor> = input.lines().map(str::parse).collect::<Result<_, _>>()?;

    const Y: i64 = 2000000;
    let Range(from, to) = sensors
        .iter()
        .filter_map(|s| s.xrange(Y))
        .reduce(|r1, r2| Range(min(r1.0, r2.0), max(r1.1, r2.1)))
        .expect("invalid input");

    let cnt = (from..=to)
        .filter(|x| {
            sensors
                .iter()
                .any(|s| s.in_range(Point(*x, Y)) && Point(*x, Y) != s.beacon)
        })
        .count();

    Ok(cnt)
}

pub fn part2(input: &str) -> crate::Result<i64> {
    let sensors: Vec<Sensor> = input.lines().map(str::parse).collect::<Result<_, _>>()?;
    const RANGE: Range = Range(0, 4000000);
    for sensor in &sensors {
        if let Some(p) = sensor.candidates().find(|pos| {
            RANGE.contains(pos.0)
                && RANGE.contains(pos.1)
                && sensors.iter().all(|s| !s.in_range(*pos))
        }) {
            return Ok(p.0 * 4000000 + p.1);
        }
    }
    panic!("oops");
}
