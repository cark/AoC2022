pub const INPUT: &str = include_str!("input.txt");

use std::collections::{HashMap, VecDeque};

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
    bfs(&cave.valves, cave.start_valve_id, max_ppm, 30)
}

fn bfs(valves: &[Valve], start_id: ValveId, max_ppm: i32, max_time: i32) -> i32 {
    let mut queue = VecDeque::new();
    queue.push_back(State::new(start_id));
    let mut weights = Vec::with_capacity(valves.len());
    weights.resize(valves.len(), i32::MIN);
    let mut best_p = 0;
    //let mut i = 0;
    while let Some(mut state) = queue.pop_front() {
        //i += 1;
        state.inc_time();
        if state.p > best_p {
            best_p = state.p
        }
        if state.t >= max_time {
            continue;
        }
        if state.ppm >= max_ppm {
            queue.push_back(state);
            continue;
        }
        valves[state.valve].edges.iter().for_each(|&id| {
            let new_state = State { valve: id, ..state };
            if weights[new_state.valve] < new_state.ppm {
                weights[new_state.valve] = new_state.ppm;
                queue.push_back(new_state);
            }
        });
        let (rate, mask) = (valves[state.valve].rate, 1usize << state.valve);
        if rate > 0 && (state.activated & mask) == 0 {
            state.ppm += rate;
            state.activated |= mask;
            queue.push_back(state);
        }
    }
    //println!("{i}");
    best_p
}

#[derive(Debug, Clone, Copy)]
struct State {
    valve: ValveId,
    activated: usize,
    ppm: i32,
    p: i32,
    t: i32,
}

impl State {
    fn new(valve: ValveId) -> Self {
        Self {
            activated: 0,
            p: 0,
            ppm: 0,
            t: 0,
            valve,
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
    //2181
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(TEST_INPUT, (20, 20)), 56000011);
    //     //assert_eq!(part2(INPUT, (4000000, 4000000)), 13743542639657);
    // }
}
