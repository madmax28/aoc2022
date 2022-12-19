use std::{cmp::max, collections::HashMap};

type Cost = [i32; 3];
type Blueprint = [Cost; 4];

fn parse(s: &str) -> Vec<Blueprint> {
    let num = |s: &str| -> i32 {
        s.chars()
            .skip_while(|c| !c.is_ascii_digit())
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap()
    };

    s.lines()
        .map(|line| {
            let robots: Vec<&str> = line.split("Each").collect();
            let ore = [num(robots[1]), 0, 0];
            let clay = [num(robots[2]), 0, 0];
            let parts: Vec<&str> = robots[3].split("and").collect();
            let obsidian = [num(parts[0]), num(parts[1]), 0];
            let parts: Vec<&str> = robots[4].split("and").collect();
            let geode = [num(parts[0]), 0, num(parts[1])];
            [ore, clay, obsidian, geode]
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Factory {
    robots: [i32; 4],
    materials: [i32; 4],
}

impl Factory {
    fn new() -> Self {
        Self {
            robots: [1, 0, 0, 0],
            materials: [0, 0, 0, 0],
        }
    }

    fn has(&self, cost: &Cost) -> bool {
        (0..3).all(|i| self.materials[i] >= cost[i])
    }

    fn take(&mut self, cost: &Cost) {
        (0..3).for_each(|i| self.materials[i] -= cost[i])
    }

    fn collect(&mut self) {
        (0..4).for_each(|i| self.materials[i] += self.robots[i])
    }

    fn same_or_better(&self, other: &Factory) -> bool {
        self.materials[0] >= other.materials[0]
            && self.materials[1] >= other.materials[1]
            && self.materials[2] >= other.materials[2]
            && self.materials[3] >= other.materials[3]
    }
}

fn search(blueprint: &Blueprint, limit: u32) -> i32 {
    let robot_limits = blueprint.iter().fold([0, 0, 0], |mut acc, cost| {
        (0..3).for_each(|i| acc[i] = max(acc[i], cost[i]));
        acc
    });

    let mut res = 0;
    let mut frontier: Vec<Factory> = vec![Factory::new()];
    for _ in 0..limit {
        // frontier for next iteration, bucketized by robot count for faster insertion
        let mut best: HashMap<[i32; 4], Vec<Factory>> = HashMap::new();
        // filters out strictly worse options
        let mut insert = |factory: Factory| {
            let candiates = best.entry(factory.robots).or_default();

            let mut i = 0;
            while i < candiates.len() {
                if candiates[i].same_or_better(&factory) {
                    return;
                }

                if factory.same_or_better(&candiates[i]) {
                    candiates.remove(i);
                    continue;
                }

                i += 1;
            }

            candiates.push(factory);
        };

        for factory in &frontier {
            for (i, costs) in blueprint.iter().enumerate() {
                // if already producing more of this per minute than any robot costs, stop building
                // these robots (except geode robots)
                if i != 3 && factory.robots[i] >= robot_limits[i] {
                    continue;
                }

                if factory.has(costs) {
                    let mut f = factory.clone();
                    f.take(costs);
                    f.collect();
                    f.robots[i] += 1;
                    insert(f);
                }
            }

            let mut f = factory.clone();
            f.collect();
            res = max(res, f.materials[3]);
            insert(f);
        }
        frontier = best.into_values().flatten().collect();
    }

    res
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let mut sum = 0;
    for (i, blueprint) in parse(input).iter().enumerate() {
        sum += (1 + i as i32) * search(blueprint, 24);
    }
    Ok(sum)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let mut product = 1;
    for blueprint in parse(input).iter().take(3) {
        product *= search(blueprint, 32);
    }
    Ok(product)
}
