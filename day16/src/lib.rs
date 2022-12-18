pub const INPUT: &str = include_str!("input.txt");

use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

type ValveId = usize;

struct Valve {
    edges: Vec<ValveId>,
    rate: i32,
}

struct Cave {
    valves: Vec<Valve>,
    start_valve_id: ValveId,
}

pub fn part1(input: &str) -> i32 {
    let cave = parse(input);
    let max_ppm = cave.valves.iter().map(|v| v.rate).sum();
    bfs(&cave.valves, cave.start_valve_id, max_ppm, 30, 1)
}

fn bfs(
    valves: &[Valve],
    start_id: ValveId,
    max_ppm: i32,
    max_time: i32,
    agent_count: usize,
) -> i32 {
    let mut queue = VecDeque::new();
    let state = Rc::new(RefCell::new(State::new()));
    for i in 0..agent_count {
        queue.push_back(Agent::new(start_id, state.clone()));
    }
    let mut weights = Vec::with_capacity(valves.len());
    weights.resize(valves.len(), i32::MIN);
    let mut best_p = 0;
    //let mut i = 0;
    while let Some(mut agent) = queue.pop_front() {
        //i += 1;
        agent.inc_time();
        if agent.p > best_p {
            best_p = agent.p
        }
        if agent.t >= max_time {
            continue;
        }
        if agent.ppm >= max_ppm {
            queue.push_back(agent);
            continue;
        }
        valves[agent.valve].edges.iter().for_each(|&id| {
            let new_state = State { valve: id, ..agent };
            if weights[new_state.valve] < new_state.ppm {
                weights[new_state.valve] = new_state.ppm;
                queue.push_back(new_state);
            }
        });
        let (rate, mask) = (valves[agent.valve].rate, 1usize << agent.valve);
        if rate > 0 && (agent.activated & mask) == 0 {
            agent.ppm += rate;
            agent.activated |= mask;
            queue.push_back(agent);
        }
    }
    //println!("{i}");
    best_p
}

struct Agent {
    valve: ValveId,
    state: Rc<RefCell<State>>,
}

impl Agent {
    fn new(valve: ValveId, state: Rc<RefCell<State>>) -> Self {
        Self { valve, state }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    activated: usize,
    ppm: i32,
    p: i32,
    t: i32,
}

impl State {
    fn new() -> Self {
        Self {
            activated: 0,
            p: 0,
            ppm: 0,
            t: 0,
        }
    }

    fn inc_time(&mut self) {
        self.t += 1;
        self.p += self.ppm;
    }
}

fn parse(input: &str) -> Cave {
    let mut start_valve_id = 0;
    let mut valves = Vec::with_capacity(200);
    let mut index_edges = Vec::with_capacity(200);
    let mut name_indexes = HashMap::new(); //HashMap::with_capacity(200);
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
    for (i, edges) in index_edges.into_iter().enumerate() {
        valves[i].edges = edges.iter().map(|&name| name_indexes[name]).collect();
    }
    Cave {
        start_valve_id,
        valves,
    }
}

// fn pairs<T: Copy>(values: &[T]) -> Vec<Vec<T>> {
//     let mut result = Vec::new();
//     if values.len() == 1 {
//         result.push(vec![values[0]]);
//     } else if values.len() > 1 {
//         for i in 0..values.len() - 1 {
//             for j in i + 1..values.len() {
//                 result.push(vec![values[i], values[j]]);
//             }
//         }
//     }
//     result
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_parse() {
        let cave = parse(TEST_INPUT);
        assert_eq!(cave.valves.len(), 10);
        assert_eq!(cave.valves.iter().map(|v| v.edges.len()).sum::<usize>(), 20);
    }

    #[test]
    fn test_part1_test() {
        assert_eq!(part1(TEST_INPUT), 1651);
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(part1(INPUT), 2181);
    }

    // #[test]
    // fn test_pairs() {
    //     let data: [usize; 0] = [];
    //     let result: [[usize; 2]; 0] = [];
    //     assert_eq!(pairs::<usize>(&data), result);
    //     assert_eq!(pairs(&[1]), [[1]]);
    //     assert_eq!(pairs(&[1, 2]), [[1, 2]]);
    //     assert_eq!(pairs(&[1, 2, 3]), [[1, 2], [1, 3], [2, 3]]);
    //     assert_eq!(
    //         pairs(&[1, 2, 3, 4]),
    //         [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
    //     );
    // }
    //2181
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(TEST_INPUT, (20, 20)), 56000011);
    //     //assert_eq!(part2(INPUT, (4000000, 4000000)), 13743542639657);
    // }
}
