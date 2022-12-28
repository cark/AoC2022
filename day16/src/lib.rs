use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, VecDeque},
    iter::once,
};

pub const INPUT: &str = include_str!("input.txt");

type ValveId = usize;

#[derive(Debug)]
struct Valve<Edge> {
    edges: Vec<Edge>,
    rate: i32,
}

#[derive(Debug)]
pub struct Cave {
    valves: Vec<Valve<DistantEdge>>,
    start_valve_id: ValveId,
}

#[derive(Debug)]
struct DistantEdge {
    valve_id: ValveId,
    dist: i32,
}

impl Cave {
    pub fn parse(input: &str) -> Self {
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
                    rate,
                })
            });
        // edge fixup
        for (i, edges) in index_edges.into_iter().enumerate() {
            valves[i].edges = edges.iter().map(|&name| name_indexes[name]).collect();
        }

        // we're only interested in "good" valves
        let good_valve_ids = once(start_valve_id)
            .chain((0..valves.len()).filter(|&id| valves[id].rate > 0))
            .collect::<Vec<_>>();
        let mut new_id = 0;
        let old_to_new = (0..valves.len())
            .map(|i| {
                if valves[i].rate > 0 {
                    new_id += 1;
                    Some(new_id)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let good_start = 0;
        let good_valves = good_valve_ids
            .iter()
            .map(|&valve_id| {
                let mut result = find_distant_edges(&valves, valve_id);
                result
                    .edges
                    .iter_mut()
                    .for_each(|de| de.valve_id = old_to_new[de.valve_id].unwrap());
                result
            })
            .collect::<Vec<_>>();

        Self {
            start_valve_id: good_start,
            valves: good_valves,
        }
    }
}

/// finds the distances from start valve to valves with pressure rate
fn find_distant_edges(valves: &[Valve<ValveId>], start_id: ValveId) -> Valve<DistantEdge> {
    let mut queue = VecDeque::new();
    let mut visited: usize = 0;
    let mut result = Vec::with_capacity(valves.len());
    queue.push_back((0, start_id));
    while let Some((dist, valve_id)) = queue.pop_front() {
        let valve = &valves[valve_id];
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
    Valve {
        rate: valves[start_id].rate,
        edges: result,
    }
}

pub fn part1(cave: &Cave) -> i32 {
    let acceptable = (2u32.pow(cave.valves.len() as u32) - 1) & !1;
    let mut tentative_pressures = vec![0; cave.valves.len()];
    best_pressure(
        &cave.valves,
        cave.start_valve_id,
        acceptable,
        30,
        &mut tentative_pressures,
    )
}

pub fn part2(cave: &Cave) -> i32 {
    let combination_count = 2u32.pow(cave.valves.len() as u32);
    let combination_mask = combination_count as u32 - 1;
    let mut tentative_pressures = vec![0; cave.valves.len()];
    let mut cache = HashMap::new();
    (0..combination_count)
        .map(|good_valves_mask| {
            // all combination of bits for good valves... and their converse for the elephant
            [
                good_valves_mask & !1,
                (!good_valves_mask) & combination_mask & !1,
            ]
            .into_iter()
            // get best pressure for this combination, both for elephant and me
            .map(|acceptable| {
                if let Some(&result) = cache.get(&(cave.start_valve_id, acceptable)) {
                    result
                } else {
                    let result = best_pressure(
                        &cave.valves,
                        cave.start_valve_id,
                        acceptable,
                        26,
                        &mut tentative_pressures,
                    );
                    cache.insert((cave.start_valve_id, acceptable), result);
                    result
                }
            })
            // sum those best pressures
            .sum()
        })
        .max()
        .unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Agent {
    id: ValveId,
    acceptable: u32,
    ppm: i32,
    p: i32,
    time_left: i32,
}

fn best_pressure(
    valves: &[Valve<DistantEdge>],
    start: ValveId,
    acceptable: u32,
    time: i32,
    tentative_pressures: &mut [i32],
) -> i32 {
    tentative_pressures.iter_mut().for_each(|p| *p = 0);
    let mut queue = BinaryHeap::new();
    queue.push(Agent {
        id: start,
        ppm: 0,
        p: 0,
        time_left: time,
        acceptable,
    });
    while let Some(agent) = queue.pop() {
        let valve = &valves[agent.id];
        valve.edges.iter().for_each(|de| {
            let mask = 1 << de.valve_id;
            let new_time_left = agent.time_left - de.dist - 1;
            if agent.acceptable & mask != 0 && new_time_left >= 0 {
                let new_valve = &valves[de.valve_id];
                let eventual_p =
                    agent.p + agent.ppm * agent.time_left + new_valve.rate * new_time_left;
                if eventual_p > tentative_pressures[de.valve_id] {
                    tentative_pressures[de.valve_id] = eventual_p;
                    queue.push(Agent {
                        ppm: agent.ppm + new_valve.rate,
                        p: agent.p + agent.ppm * (de.dist + 1),
                        time_left: new_time_left,
                        acceptable: agent.acceptable & !mask,
                        id: de.valve_id,
                    })
                }
            }
        });
    }
    tentative_pressures.iter().copied().max().unwrap()
}

impl Ord for Agent {
    fn cmp(&self, other: &Self) -> Ordering {
        let result = self.time_left.cmp(&other.time_left);
        if let Ordering::Equal = result {
            self.ppm.cmp(&other.ppm)
        } else {
            result
        }
    }
}

impl PartialOrd for Agent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Agent {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.ppm == other.ppm
            && self.p == other.p
            && self.time_left == other.time_left
    }
}

impl Eq for Agent {
    fn assert_receiver_is_total_eq(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_silly() {
        let mut queue = BinaryHeap::new();
        queue.push(Agent {
            id: 0,
            ppm: 0,
            p: 0,
            time_left: 2,
            acceptable: 0,
        });
        queue.push(Agent {
            id: 0,
            ppm: 2,
            p: 0,
            time_left: 3,
            acceptable: 0,
        });
        queue.push(Agent {
            id: 0,
            ppm: 1,
            p: 0,
            time_left: 3,
            acceptable: 0,
        });
        let Some(a) = queue.pop() else { panic!() };
        assert_eq!(a.time_left, 3);
        assert_eq!(a.ppm, 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Cave::parse(TEST_INPUT)), 1707);
        assert_eq!(part2(&Cave::parse(INPUT)), 2824);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&Cave::parse(TEST_INPUT)), 1651);
        assert_eq!(part1(&Cave::parse(INPUT)), 2181);
    }
}
