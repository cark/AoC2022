use std::collections::{HashMap, VecDeque};

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
            valves,
        };

        // we're only interested in edges with pressure, and the distance to these
        // plus our starting valve
        let interesting_valves = std::iter::once(start_valve_id)
            .chain((0..result.valves.len()).filter(|&id| result.valves[id].rate > 0))
            .collect::<Vec<_>>();
        for valve_id in interesting_valves {
            let distant_edges = result.find_distant_edges(valve_id);
            result.valves[valve_id].distant_edges = distant_edges;
        }
        // result.valves_with_pressure = result.interesting_valves().count() as u32;
        result
    }

    /// finds the valves with rate connected to the input starting valve, and their distance
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

    fn best_pressure(valves: &[Valve], start_id: ValveId, time: i32) -> i32 {
        fn dfs(
            valves: &[Valve],
            id: ValveId,
            time_left: i32,
            mut visited: usize,
            ppm: i32,
            pressure: i32,
            mut pressures: Vec<i32>,
        ) -> Vec<i32> {
            visited |= 1 << id;
            let mut could_move = false;
            for de in valves[id].distant_edges.iter() {
                let mask = 1 << de.valve_id;
                if visited & mask != 0 {
                    continue;
                }
                let new_time_left = time_left - de.dist - 1;
                if new_time_left < 0 {
                    continue;
                }
                let new_pressure = pressure + ppm * (de.dist + 1);
                let new_ppm = ppm + valves[de.valve_id].rate;
                could_move = true;
                pressures = dfs(
                    valves,
                    de.valve_id,
                    new_time_left,
                    visited,
                    new_ppm,
                    new_pressure,
                    pressures,
                );
            }
            if !could_move {
                let new_pressure = pressure + ppm * time_left;
                pressures.push(new_pressure);
            }
            pressures
        }
        let pressures = dfs(
            valves,
            start_id,
            time,
            0 << start_id,
            0,
            0,
            Vec::with_capacity(1024),
        );
        println!("{}", pressures.len());
        println!("{}", !0usize);
        *pressures.iter().max().unwrap()
    }
}

pub fn part1(input: &str) -> i32 {
    let cave = Cave::parse(input);
    Cave::best_pressure(&cave.valves, cave.start_valve_id, 30)
}

pub fn part2(input: &str) -> i32 {
    let cave = Cave::parse(input);
    todo!()
}

// #[derive(Debug)]
// struct Valve {
//     edges: Vec<ValveId>,
//     distant_edges: Vec<DistantEdge>,
//     rate: i32,
// }

// #[derive(Debug)]
// struct Cave {
//     valves: Vec<Valve>,
//     start_valve_id: ValveId,
//     valves_with_pressure: u32,
// }

// #[derive(Debug)]
// struct DistantEdge {
//     valve_id: ValveId,
//     dist: i32,
// }

// impl Cave {
//     fn interesting_valves(&self) -> impl Iterator<Item = ValveId> + '_ {
//         self.valves
//             .iter()
//             .enumerate()
//             .filter_map(|(id, valve)| if valve.rate > 0 { Some(id) } else { None })
//     }

//     fn find_distant_edges(&self, start_id: ValveId) -> Vec<DistantEdge> {
//         let mut queue = VecDeque::new();
//         let mut visited: usize = 0;
//         let mut result = Vec::with_capacity(self.valves.len());
//         queue.push_back((0, start_id));
//         while let Some((dist, valve_id)) = queue.pop_front() {
//             let valve = &self.valves[valve_id];
//             if valve.rate > 0 && valve_id != start_id {
//                 result.push(DistantEdge { dist, valve_id });
//             }
//             for &new_id in valve.edges.iter() {
//                 let mask = 1 << new_id;
//                 if visited & mask == 0 {
//                     visited |= mask;
//                     queue.push_back((dist + 1, new_id));
//                 }
//             }
//         }
//         result
//     }

// }

// #[derive(Debug, Clone, Copy)]
// struct State {
//     valve_id: ValveId,
//     next_valve_id: ValveId,
//     activated: usize,
//     ppm: i32,
//     p: i32,
//     t: i32,
//     time_to_valve: i32,
// }

// impl State {
//     fn new(at_valve: ValveId) -> Self {
//         Self {
//             valve_id: at_valve,
//             next_valve_id: at_valve,
//             time_to_valve: 0,
//             activated: 0,
//             p: 0,
//             ppm: 0,
//             t: 0,
//         }
//     }

//     fn inc_time(&mut self) {
//         self.t += 1;
//         self.p += self.ppm;
//         self.time_to_valve = (self.time_to_valve - 1).max(0);
//         if self.time_to_valve == 0 {
//             self.valve_id = self.next_valve_id;
//         }
//     }
// }

// pub fn part1(input: &str) -> i32 {
//     let cave = Cave::parse(input);

//     bfs(
//         &cave.valves,
//         cave.start_valve_id,
//         30,
//         cave.valves_with_pressure,
//     )
// }

// fn bfs(valves: &[Valve], start_id: ValveId, max_time: i32, max_open_valves: u32) -> i32 {
//     let mut queue = VecDeque::new();
//     let state = State::new(start_id);
//     queue.push_back(state);
//     let mut ppms = Vec::with_capacity(valves.len());
//     ppms.resize(valves.len(), 0);
//     let mut best_p = 0;
//     let mut best_state = state;
//     let mut i = 0;
//     let mut pruned = 0;
//     while let Some(mut state) = queue.pop_front() {
//         i += 1;
//         state.inc_time();
//         if state.p > best_p {
//             best_p = state.p;
//             best_state = state;
//         }
//         let (rate, mask) = (valves[state.valve_id].rate, 1usize << state.valve_id);
//         if state.t >= max_time {
//             continue;
//         } else if state.time_to_valve > 0 || state.activated.count_ones() == max_open_valves {
//             queue.push_back(state);
//         } else if rate > 0 && (state.activated & mask) == 0 {
//             state.ppm += rate;
//             state.activated |= mask;
//             ppms[state.valve_id] = state.ppm;
//             queue.push_back(state);
//         } else {
//             for dist_edge in valves[state.valve_id].distant_edges.iter() {
//                 let mask = 1usize << dist_edge.valve_id;
//                 if (state.activated & mask) != 0 {
//                     continue;
//                 }
//                 if ppms[dist_edge.valve_id] > state.ppm + valves[dist_edge.valve_id].rate {
//                     pruned += 1;
//                     continue;
//                 }
//                 let mut new_state = state;
//                 new_state.next_valve_id = dist_edge.valve_id;
//                 new_state.time_to_valve = dist_edge.dist;
//                 queue.push_back(new_state);
//             }
//         }
//     }
//     println!(
//         "iterations: {i}, activations: {}, pruned: {}",
//         best_state.activated.count_ones(),
//         pruned
//     );
//     best_p
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_parse() {
        let cave = Cave::parse(TEST_INPUT);
        assert_eq!(cave.valves.len(), 10);
        assert_eq!(cave.valves.iter().map(|v| v.edges.len()).sum::<usize>(), 20);
        // println!("{:#?}", cave);
        // assert!(false);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1651);
        assert_eq!(part1(INPUT), 2181);
        // assert!(false);
    }
}
