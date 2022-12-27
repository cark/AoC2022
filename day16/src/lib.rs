use std::{
    collections::{HashMap, VecDeque},
    iter::once,
};

pub const INPUT: &str = include_str!("input.txt");

type ValveId = usize;

#[derive(Debug)]
struct Valve {
    edges: Vec<ValveId>,
    distant_edges: Vec<DistantEdge>,
    rate: i32,
}

#[derive(Debug)]
struct Cave {
    valves: Vec<Valve>,
    good_valves: Vec<ValveId>,
    start_valve_id: ValveId,
}

#[derive(Debug)]
struct DistantEdge {
    valve_id: ValveId,
    dist: i32,
}

impl Cave {
    fn parse(input: &str) -> Self {
        let mut start_valve_id = 0;
        let mut valves = Vec::with_capacity(200);
        let mut index_edges = Vec::with_capacity(200);
        let mut name_indexes = HashMap::new();
        input
            .lines()
            .filter(|line| !line.is_empty())
            .for_each(|line| {
                let mut parts = line.split(';');
                let mut tokens = parts.next().unwrap().split_whitespace();
                let name = tokens.nth(1).unwrap();
                let rate = tokens
                    .nth(2)
                    .and_then(|s| s.split('=').nth(1))
                    .map(|s| s.parse::<i32>().unwrap())
                    .unwrap();
                let edges = parts
                    .next()
                    .and_then(|s| {
                        s.strip_prefix(" tunnels lead to valves ")
                            .or(s.strip_prefix(" tunnel leads to valve "))
                    })
                    .map(|s| s.split(", ").collect::<Vec<_>>())
                    .unwrap();
                index_edges.push(edges);
                name_indexes.insert(name, valves.len());
                if name == "AA" {
                    start_valve_id = valves.len()
                }
                valves.push(Valve {
                    edges: vec![],
                    distant_edges: vec![],
                    rate,
                })
            });
        // edge fixup
        for (i, edges) in index_edges.into_iter().enumerate() {
            valves[i].edges = edges.iter().map(|&name| name_indexes[name]).collect();
        }

        let mut result = Self {
            start_valve_id,
            good_valves: (0..valves.len())
                .filter(|&id| valves[id].rate > 0)
                .collect(),
            valves,
        };

        // we're only interested in edges with pressure, and the distance to these
        // plus our starting valve
        for valve_id in once(start_valve_id).chain(result.good_valves.iter().copied()) {
            let distant_edges = result.find_distant_edges(valve_id);
            result.valves[valve_id].distant_edges = distant_edges;
        }
        result
    }

    /// finds the distances from start valve to valves with pressure rate
    fn find_distant_edges(&self, start_id: ValveId) -> Vec<DistantEdge> {
        let mut queue = VecDeque::new();
        let mut visited: usize = 0;
        let mut result = Vec::with_capacity(self.valves.len());
        queue.push_back((0, start_id));
        while let Some((dist, valve_id)) = queue.pop_front() {
            let valve = &self.valves[valve_id];
            if valve.rate > 0 && valve_id != start_id {
                result.push(DistantEdge { dist, valve_id });
            }
            for &new_id in valve.edges.iter() {
                let mask = 1 << new_id;
                if visited & mask == 0 {
                    visited |= mask;
                    queue.push_back((dist + 1, new_id));
                }
            }
        }
        result
    }
}

type BestPressureParams = (ValveId, usize, i32);

fn best_pressure(
    valves: &[Valve],
    cache: &mut HashMap<BestPressureParams, i32>,
    params @ (id, acceptable, time_left): (ValveId, usize, i32),
) -> i32 {
    if let Some(result) = cache.get(&params) {
        *result
    } else {
        let valve = &valves[id];
        let result = valve
            .distant_edges
            .iter()
            .filter_map(|de| {
                let mask = 1 << de.valve_id;
                let new_time_left = time_left - de.dist - 1;
                (acceptable & mask != 0 && new_time_left >= 0).then(|| {
                    valve.rate * time_left
                        + best_pressure(
                            valves,
                            cache,
                            (de.valve_id, acceptable & !mask, new_time_left),
                        )
                })
            })
            .max()
            .unwrap_or(valve.rate * time_left);
        cache.insert(params, result);
        result
    }
}

pub fn part1(input: &str) -> i32 {
    let cave = Cave::parse(input);
    let acceptable = cave
        .good_valves
        .iter()
        .fold(0usize, |result, i| result | (1 << i));
    let mut cache = HashMap::new();
    best_pressure(
        &cave.valves,
        &mut cache,
        (cave.start_valve_id, acceptable, 30),
    )
}

pub fn part2(input: &str) -> i32 {
    let cave = Cave::parse(input);
    let combination_count = 2usize.pow(cave.good_valves.len() as u32);
    let combination_mask = combination_count - 1;
    let mut cache = HashMap::new();
    (0..combination_count)
        .map(|good_valves_mask| {
            [good_valves_mask, (!good_valves_mask) & combination_mask]
                .iter()
                .map(|good_valves_mask| {
                    cave.good_valves
                        .iter()
                        .enumerate()
                        .fold(0usize, |result, (i, id)| {
                            if good_valves_mask & (1 << i) != 0 {
                                result | (1 << id)
                            } else {
                                result
                            }
                        })
                })
                .map(|acceptable| {
                    best_pressure(
                        &cave.valves,
                        &mut cache,
                        (cave.start_valve_id, acceptable, 26),
                    )
                })
                .sum()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1707);
        assert_eq!(part2(INPUT), 2824);
    }

    #[test]
    fn test_parse() {
        let cave = Cave::parse(TEST_INPUT);
        assert_eq!(cave.valves.len(), 10);
        assert_eq!(cave.valves.iter().map(|v| v.edges.len()).sum::<usize>(), 20);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1651);
        assert_eq!(part1(INPUT), 2181);
    }
}
