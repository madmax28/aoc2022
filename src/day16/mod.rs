use std::{
    cmp::{max, min, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

#[derive(Debug, Clone)]
struct Valve<'a> {
    pressure: i32,
    tunnels: HashMap<&'a str, i32>,
}

#[derive(Debug, Clone)]
struct Map<'a> {
    valves: HashMap<&'a str, Valve<'a>>,
}

impl<'a> Map<'a> {
    fn parse(s: &'a str) -> crate::Result<Self> {
        let mut valves = HashMap::new();
        for line in s.lines() {
            let valve = &line[6..=7];
            let pressure = line[23..]
                .chars()
                .take_while(char::is_ascii_digit)
                .collect::<String>()
                .parse()?;
            let tunnels = line
                .split("to valve")
                .nth(1)
                .expect("no edges")
                .trim_matches(|c: char| c.is_lowercase() || c.is_whitespace())
                .split(", ")
                .map(|valve| (valve, 1))
                .collect();

            valves.insert(valve, Valve { pressure, tunnels });
        }

        Ok(Map { valves })
    }

    fn cost(&self, from: &str, to: &str) -> i32 {
        let mut q = BinaryHeap::new();
        q.push(Reverse((0, from)));
        loop {
            let Reverse((cost, at)) = q.pop().unwrap();
            if at == to {
                return cost;
            }

            for (pos, edge_cost) in &self.valves[at].tunnels {
                q.push(Reverse((cost + edge_cost, pos)));
            }
        }
    }

    fn fill_costs(&mut self) {
        let keys: Vec<_> = self.valves.keys().copied().collect();
        for from in &keys {
            for to in &keys {
                if from == to {
                    continue;
                }

                let cost = self.cost(from, to);
                self.valves.get_mut(from).unwrap().tunnels.insert(to, cost);
            }
        }
    }

    fn flatten(&mut self) {
        let keys: Vec<_> = self.valves.keys().copied().filter(|k| *k != "AA").collect();
        for key in keys {
            if self.valves[key].pressure > 0 {
                continue;
            }

            let valve = self.valves.remove(key).unwrap();
            for (from_id, from_cost) in &valve.tunnels {
                for (to_id, to_cost) in &valve.tunnels {
                    self.valves.get_mut(from_id).unwrap().tunnels.remove(key);
                    self.valves.get_mut(to_id).unwrap().tunnels.remove(key);

                    if from_id == to_id {
                        continue;
                    }

                    self.valves
                        .get_mut(from_id)
                        .unwrap()
                        .tunnels
                        .entry(to_id)
                        .and_modify(|cost| *cost = min(*cost, from_cost + to_cost))
                        .or_insert(from_cost + to_cost);

                    self.valves
                        .get_mut(to_id)
                        .unwrap()
                        .tunnels
                        .entry(from_id)
                        .and_modify(|cost| *cost = min(*cost, from_cost + to_cost))
                        .or_insert(from_cost + to_cost);
                }
            }
        }
    }

    fn into_vec(self) -> Vec<(i32, Vec<i32>)> {
        let (mut keys, mut valves): (Vec<&str>, Vec<Valve>) = self.valves.into_iter().unzip();

        // put "AA" in front
        let idx = keys
            .iter()
            .enumerate()
            .find(|(_, k)| *k == &"AA")
            .unwrap()
            .0;
        keys.swap(0, idx);
        valves.swap(0, idx);

        let valves: Vec<(i32, Vec<i32>)> = valves
            .into_iter()
            .map(|valve| {
                let tunnels = keys
                    .iter()
                    .map(|key| *valve.tunnels.get(key).unwrap_or(&0))
                    .collect::<Vec<_>>();
                (valve.pressure, tunnels)
            })
            .collect();

        valves
    }
}

#[derive(Debug, Clone)]
struct State {
    time: i32,
    released: i32,
    flow: i32,
}

impl State {
    fn new() -> Self {
        State {
            time: 0,
            released: 0,
            flow: 0,
        }
    }
}

fn search(
    valves: &[(i32, Vec<i32>)],
    from: usize,
    rest: &mut VecDeque<usize>,
    state: State,
    limit: i32,
) -> i32 {
    if rest.is_empty() {
        return state.released + state.flow * (limit - state.time);
    }

    let mut best = 0;
    for _ in 0..rest.len() {
        let to = rest.pop_front().unwrap();

        let cost = valves[from].1[to] + 1;
        if state.time + cost >= limit {
            best = max(best, state.released + state.flow * (limit - state.time));
        } else {
            let mut state = state.clone();
            state.time += cost;
            state.released += state.flow * cost;
            state.flow += valves[to].0;
            best = max(best, search(valves, to, rest, state, limit));
        }

        rest.push_back(to);
    }
    best
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let mut map = Map::parse(input)?;
    map.flatten();
    map.fill_costs();
    let valves = map.into_vec();

    Ok(search(
        &valves,
        0,
        &mut (1..valves.len()).collect(),
        State::new(),
        30,
    ))
}

fn search_ele(
    valves: &[(i32, Vec<i32>)],
    me: &mut VecDeque<usize>,
    ele: &mut VecDeque<usize>,
    visited: &mut HashSet<Vec<usize>>,
) -> i32 {
    let mut key: Vec<usize> = me.iter().copied().collect();
    key.sort();
    if !visited.insert(key) {
        return 0;
    }

    let mut best =
        search(valves, 0, me, State::new(), 26) + search(valves, 0, ele, State::new(), 26);

    for _ in 0..me.len() {
        ele.push_back(me.pop_front().unwrap());
        best = max(best, search_ele(valves, me, ele, visited));
        me.push_back(ele.pop_back().unwrap());
    }
    best
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let mut map = Map::parse(input)?;
    map.flatten();
    map.fill_costs();
    let valves = map.into_vec();

    let mut me = (1..valves.len()).collect();
    let mut ele = VecDeque::new();
    Ok(search_ele(&valves, &mut me, &mut ele, &mut HashSet::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        assert_eq!(1651, part1(input).unwrap());
        assert_eq!(1707, part2(input).unwrap());
    }
}
