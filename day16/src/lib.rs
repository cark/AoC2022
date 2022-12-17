pub const INPUT: &str = include_str!("input.txt");

use std::collections::{HashMap, VecDeque};

type ValveId = usize;

struct Valve {
    edges: Vec<ValveId>,
    rate: i32,
    //    name: String,
}

struct Cave {
    valves: Vec<Valve>,
    start_valve_id: ValveId,
}


pub fn part1(input: &str) -> i32 {
    let cave = parse(input);
    let destinations = std::iter::once(cave.start_valve_id)
        .chain(
            cave.valves
                .iter()
                .enumerate()
                .filter_map(|(i, v)| if v.rate > 0 { Some(i) } else { None }),
        )
        .collect::<Vec<_>>();
    let all_paths = Vec::with_capacity(1000);
    for i in 0..destinations.len() - 1 {
	for j in i+1..destinations.len() {
	    let pair: usize = (1 << i) | (1 << j);
	    
	}
    }

    todo!()
    // bfs(&cave.valves, cave.start_valve_id)
    // let mut hit_miss: (u64, u64) = (0, 0);
    // dfs(
    //     &cave.valves,
    //     State::new(cave.start_valve_id, 0, 0, 0),
    //     &mut hit_miss,
    // )
    // let mut hit_miss = (0, 0);
    // dfs(
    //     &cave.valves,
    //     State::new(cave.start_valve_id, 0, 0, 0),
    //     &mut HashMap::new(),
    //     &mut hit_miss,
    // )
}

// #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
// struct State {
//     valve: ValveId,
//     time: i32,
//     pressure: i32,
//     ppm: i32,
//     open_valves: u64,
//     //    visited: Vec<ValveId>,
// }

// impl State {
//     fn new(valve: ValveId, time: i32, pressure: i32, ppm: i32) -> Self {
//         State {
//             valve,
//             time,
//             pressure,
//             ppm,
//             open_valves: 0,
//             //            visited: vec![],
//         }
//     }
//     fn weight(&self) -> i32 {
//         self.pressure + (30 - self.time) * self.ppm
//     }
//     fn is_valve_open(&self) -> bool {
//         ((self.open_valves >> self.valve) & 1) != 0
//     }
//     fn open_valve(&self, rate: i32) -> Self {
//         let mut result = self.advance_time();
//         result.open_valves |= 1 << self.valve;
//         result.ppm += rate;
//         result
//     }
//     fn advance_time(&self) -> Self {
//         let mut result = self.clone();
//         result.pressure += result.ppm;
//         result.time += 1;
//         result
//     }
//     fn go(&self, to: ValveId) -> Self {
//         let mut result = self.advance_time();
//         //        result.visited.push(to);
//         result.valve = to;
//         result
//     }
// }

// , memo: &mut HashMap<State, i32>

// fn dfs(valves: &[Valve], s: State, hit_miss: &mut (u64, u64)) -> i32 {
//     hit_miss.0 += 1;
//     if (hit_miss.0 + hit_miss.1) % 10000000 == 0 {
//         println!("{hit_miss:?}");
//     }
//     if s.time == 30 {
//         s.pressure
//     } else {
//         let valve = &valves[s.valve];
//         std::iter::once(if !s.is_valve_open() && valve.rate > 0 {
//             s.open_valve(valve.rate)
//         } else {
//             s.advance_time()
//         })
//         .chain(valve.edges.iter().map(|&id| s.go(id)))
//         .map(|s| dfs(valves, s, hit_miss))
//         .max()
//         .unwrap()
//     }
// }

// fn dfs(
//     valves: &[Valve],
//     s: State,
//     memo: &mut HashMap<State, i32>,
//     hit_miss: &mut (i32, i32),
// ) -> i32 {
//     if (hit_miss.0 + hit_miss.1) % 100000 == 0 {
//         println!("{hit_miss:?}");
//     }
//     if let Some(result) = memo.get(&s) {
//         hit_miss.0 += 1;
//         *result
//     } else {
//         hit_miss.1 += 1;
//         let result = if s.time == 30 {
//             s.pressure
//         } else {
//             let valve = &valves[s.valve];
//             std::iter::once(if !s.is_valve_open() && valve.rate > 0 {
//                 s.open_valve(valve.rate)
//             } else {
//                 s.advance_time()
//             })
//             .chain(valve.edges.iter().map(|&id| s.go(id)))
//             .map(|s| dfs(valves, s, memo, hit_miss))
//             .max()
//             .unwrap()
//         };
//         memo.insert(s, result);
//         result
//     }
// }

// fn bfs(valves: &[Valve], start: ValveId) -> i32 {
//     let mut weights = Vec::with_capacity(valves.len());
//     weights.resize(valves.len(), i32::MIN);
//     let start = State::new(start, 0, 0, 0);
//     let mut best_weight = 0;
//     let mut queue = VecDeque::with_capacity(valves.len() * 2);
//     queue.push_back(start);
//     let mut iterations = 0;
//     while let Some(state) = queue.pop_front() {
//         iterations += 1;
//         let valve = &valves[state.valve];
//         let w = state.weight();
//         if w > best_weight {
//             best_weight = w;
//         }
//         if state.time == 30 {
//             continue;
//         }
//         // std::iter::once(if !state.is_valve_open() && valve.rate > 0 {
//         //     state.open_valve(valve.rate)
//         // } else {
//         //     state.advance_time()
//         // })
//         // .chain(valves[state.valve].edges.iter().map(|&next_id| State {
//         //     valve: next_id,
//         //     ..state.advance_time()
//         // }))
//         // .for_each(|state| {
//         //     let w = state.weight();
//         //     if w > weights[state.valve] {
//         //         weights[state.valve] = w;
//         //         queue.push_back(state)
//         //     }
//         // });
//         valves[state.valve]
//             .edges
//             .iter()
//             .map(|&next_id| State {
//                 valve: next_id,
//                 ..state.advance_time()
//             })
//             .chain(std::iter::once(
//                 if !state.is_valve_open() && valve.rate > 0 {
//                     state.open_valve(valve.rate)
//                 } else {
//                     state.advance_time()
//                 },
//             ))
//             .for_each(|state| {
//                 let w = state.weight();
//                 if w > weights[state.valve] {
//                     weights[state.valve] = w;
//                     queue.push_back(state)
//                 }
//             });
//     }
//     println!("best weight: {best_weight}");
//     println!("iterations: {iterations}");
//     best_weight
// }

fn parse(input: &str) -> Cave {
    let mut start_valve_id = 0;
    let mut valves = vec![];
    let mut index_edges = HashMap::with_capacity(200);
    let mut name_indexes = HashMap::with_capacity(200);
    //let mut max_pressure_per_min = 0;
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
            //max_pressure_per_min += rate;
            let edges = parts
                .next()
                .and_then(|s| {
                    //println!("{s}");
                    s.strip_prefix(" tunnels lead to valves ")
                        .or(s.strip_prefix(" tunnel leads to valve "))
                })
                .map(|s| s.split(", ").collect::<Vec<_>>())
                .unwrap();
            index_edges.insert(valves.len(), edges);
            name_indexes.insert(name.to_owned(), valves.len());
            if name == "AA" {
                start_valve_id = valves.len()
            }
            valves.push(Valve {
                // name: name.to_owned(),
                edges: vec![],
                //open: false,
                rate,
                //visits: 0,
            })
        });
    for (i, edges) in index_edges {
        valves[i].edges = edges.iter().map(|&name| name_indexes[name]).collect();
    }
    Cave {
        start_valve_id,
        //max_pressure_per_min,
        valves,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_parse() {
        let cave = parse(TEST_INPUT);
        assert_eq!(cave.valves.len(), 10);
        assert_eq!(cave.valves.iter().flat_map(|v| &v.edges).count(), 20);
    }

    // #[test]
    // fn test_part1_test() {
    //     assert_eq!(part1(TEST_INPUT), 1651);
    // }

    // #[test]
    // fn test_part1_actual() {
    //     println!("****ACTUAL DATA");
    //     assert_eq!(part1(INPUT), 2181);
    // }
    //2181
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(TEST_INPUT, (20, 20)), 56000011);
    //     //assert_eq!(part2(INPUT, (4000000, 4000000)), 13743542639657);
    // }
}
