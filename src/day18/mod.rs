use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32, i32);

impl Point {
    fn neighbors(&self) -> impl Iterator<Item = Point> {
        [
            Point(self.0 - 1, self.1, self.2),
            Point(self.0 + 1, self.1, self.2),
            Point(self.0, self.1 - 1, self.2),
            Point(self.0, self.1 + 1, self.2),
            Point(self.0, self.1, self.2 - 1),
            Point(self.0, self.1, self.2 + 1),
        ]
        .into_iter()
    }
}

type Points = HashSet<Point>;

fn parse(s: &str) -> crate::Result<Points> {
    let points = s
        .lines()
        .map(|line| -> crate::Result<Point> {
            let nums = line
                .split(',')
                .map(|num| num.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Point(nums[0], nums[1], nums[2]))
        })
        .collect::<Result<_, _>>()?;
    Ok(points)
}

#[derive(Debug)]
struct BoundingBox(Point, Point);

impl BoundingBox {
    fn from_points(points: &Points) -> Self {
        let xmin = *points.iter().map(|Point(x, _, _)| x).min().unwrap() - 1;
        let xmax = *points.iter().map(|Point(x, _, _)| x).max().unwrap() + 1;
        let ymin = *points.iter().map(|Point(_, y, _)| y).min().unwrap() - 1;
        let ymax = *points.iter().map(|Point(_, y, _)| y).max().unwrap() + 1;
        let zmin = *points.iter().map(|Point(_, _, z)| z).min().unwrap() - 1;
        let zmax = *points.iter().map(|Point(_, _, z)| z).max().unwrap() + 1;
        BoundingBox(Point(xmin, ymin, zmin), Point(xmax, ymax, zmax))
    }

    fn contains(&self, point: &Point) -> bool {
        point.0 >= self.0 .0
            && point.0 <= self.1 .0
            && point.1 >= self.0 .1
            && point.1 <= self.1 .1
            && point.2 >= self.0 .2
            && point.2 <= self.1 .2
    }
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let boulder: Points = parse(input)?;
    let surface = boulder
        .iter()
        .flat_map(|point| point.neighbors())
        .filter(|neighbor| !boulder.contains(neighbor))
        .count();
    Ok(surface)
}

pub fn part2(input: &str) -> crate::Result<usize> {
    let boulder: Points = parse(input)?;
    let bb = BoundingBox::from_points(&boulder);

    let mut outer = Points::new();
    outer.insert(bb.0);
    let mut frontier = vec![bb.0];
    while !frontier.is_empty() {
        let p = frontier.pop().unwrap();
        for neighbor in p.neighbors() {
            if bb.contains(&neighbor) && !outer.contains(&neighbor) && !boulder.contains(&neighbor)
            {
                outer.insert(neighbor);
                frontier.push(neighbor);
            }
        }
    }

    let surface = boulder
        .iter()
        .flat_map(|point| point.neighbors())
        .filter(|neighbor| outer.contains(neighbor))
        .count();
    Ok(surface)
}
