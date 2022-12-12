// turns out that dijsktra is faster than a-star even for part1

pub const INPUT: &str = include_str!("input.txt");

use std::collections::VecDeque;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Board {
    width: usize,
    height: usize,
    elevation: Vec<u8>,
    start: Pos,
    dest: Pos,
}

impl Board {
    fn parse(input: &str) -> Self {
        let mut result = Board {
            width: 0,
            height: 0,
            elevation: vec![],
            start: Pos::new(0, 0),
            dest: Pos::new(0, 0),
        };
        input
            .lines()
            .filter(|line| !line.is_empty())
            .enumerate()
            .for_each(|(y, line)| {
                result.height = y + 1;
                line.as_bytes().iter().enumerate().for_each(|(x, &byte)| {
                    if result.width <= x {
                        result.width = x + 1;
                    }
                    result.elevation.push(match byte {
                        b'S' => {
                            result.start = Pos::new(x as i32, y as i32);
                            0
                        }
                        b'E' => {
                            result.dest = Pos::new(x as i32, y as i32);
                            b'z' - b'a'
                        }
                        letter => letter - b'a',
                    });
                })
            });
        result
    }

    fn pos_index(&self, pos: Pos) -> usize {
        pos.x as usize + pos.y as usize * self.width
    }

    fn dijkstra(&self, is_dest: impl Fn(Pos) -> bool) -> i32 {
        let mut tentative_dist = Vec::with_capacity(self.elevation.len());
        tentative_dist.resize(self.elevation.len(), i32::MAX);
        tentative_dist[self.pos_index(self.dest)] = 0;
        let mut open_list = VecDeque::with_capacity(self.elevation.len());
        open_list.push_back(self.dest);
        while let Some(current) = open_list.pop_front() {
            let current_index = self.pos_index(current);
            let current_elevation = self.elevation[current_index];
            if is_dest(current) {
                return tentative_dist[current_index];
            }
            for neighbour in
                [(0, -1), (-1, 0), (1, 0), (0, 1)]
                    .into_iter()
                    .filter_map(|(dx, dy)| {
                        let x = current.x + dx;
                        let y = current.y + dy;
                        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                            Some(Pos::new(x, y))
                        } else {
                            None
                        }
                    })
            {
                let neighbour_index = self.pos_index(neighbour);
                let neighbour_elevation = self.elevation[neighbour_index];
                if (current_elevation as i32 - 1..).contains(&(neighbour_elevation as i32)) {
                    let curr_dist = tentative_dist[current_index];
                    let neighbour_dist = tentative_dist[neighbour_index];
                    if curr_dist + 1 < neighbour_dist {
                        tentative_dist[neighbour_index] = curr_dist + 1;
                        open_list.push_back(neighbour);
                    }
                }
            }
        }
        unreachable!()
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct PosCost {
    data: Pos,
    cost: i32,
}

pub fn part1(input: &str) -> i32 {
    let board = Board::parse(input);
    board.dijkstra(|pos| pos == board.start)
}

pub fn part2(input: &str) -> i32 {
    let board = Board::parse(input);
    board.dijkstra(|pos| board.elevation[board.pos_index(pos)] == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 31);
        assert_eq!(part1(INPUT), 383);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 29);
        assert_eq!(part2(INPUT), 377);
    }
}
