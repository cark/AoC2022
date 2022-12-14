pub const INPUT: &str = include_str!("input.txt");

use std::ops::{Add, Sub};

const DROP_POINT: Pos = Pos::new(500, 0);
const DOWN: Pos = Pos::new(0, 1);
const DOWN_LEFT: Pos = Pos::new(-1, 1);
const DOWN_RIGHT: Pos = Pos::new(1, 1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos::new(self.x - rhs.x, self.y - rhs.y)
    }
}

struct Reservoir {
    occupancy: Vec<bool>,
    top_left: Pos,
    bottom_right: Pos,
    size: Pos,
}

impl Reservoir {
    fn from_input(input: &str, with_bottom: bool) -> Self {
        let (mut top_left, mut bottom_right) = find_bounds(input);
        bottom_right.y += 2;
        top_left.x = 500 - bottom_right.y - 1;
        bottom_right.x = 500 + bottom_right.y + 1;
        let size = (bottom_right - top_left) + Pos::new(1, 1);
        let occupancy_size = (size.x * size.y) as usize;
        let mut occupancy = Vec::with_capacity(occupancy_size);
        occupancy.resize(occupancy_size, false);
        let mut result = Self {
            occupancy,
            top_left,
            bottom_right,
            size,
        };
        input
            .lines()
            .filter(|line| !line.is_empty())
            .flat_map(parse_path)
            .for_each(|pos| {
                result.set_occupied(pos);
            });
        if with_bottom {
            (top_left.x..=bottom_right.x).for_each(|x| {
                result.set_occupied(Pos::new(x, bottom_right.y));
            })
        }
        result
    }

    fn pos_index(&self, pos: Pos) -> usize {
        let with_offset = pos - self.top_left;
        (with_offset.x + with_offset.y * self.size.x) as usize
    }

    fn set_occupied(&mut self, pos: Pos) {
        let index = self.pos_index(pos);
        self.occupancy[index] = true;
    }

    fn is_occupied(&self, pos: Pos) -> bool {
        if self.in_bounds(pos) {
            self.occupancy[self.pos_index(pos)]
        } else {
            false
        }
    }

    fn in_bounds(&self, pos: Pos) -> bool {
        (self.top_left.x..=self.bottom_right.x).contains(&pos.x)
            && (self.top_left.y..=self.bottom_right.y).contains(&pos.y)
    }

    fn drop_sand(&mut self, mut pos: Pos) -> bool {
        loop {
            if self.is_occupied(pos) && pos == DROP_POINT {
                return false;
            }
            let new_pos = [DOWN, DOWN_LEFT, DOWN_RIGHT]
                .into_iter()
                .map(|offset| pos + offset)
                .find(|p| !self.is_occupied(*p));
            if let Some(new_pos) = new_pos {
                pos = new_pos;
                if !self.in_bounds(pos) {
                    return false;
                }
            } else {
                self.set_occupied(pos);
                return true;
            }
        }
    }

    fn simulate(&mut self) -> u64 {
        for i in 0.. {
            if !self.drop_sand(DROP_POINT) {
                return i;
            }
        }
        unreachable!()
    }
}

fn parse_vertices(line: &str) -> impl Iterator<Item = Pos> + '_ {
    line.split(" -> ").map(|vertex| {
        let mut coords = vertex.split(',').map(|v| v.parse().unwrap());
        Pos::new(coords.next().unwrap(), coords.next().unwrap())
    })
}

fn parse_path(line: &str) -> impl Iterator<Item = Pos> + '_ {
    let mut vertices = parse_vertices(line);
    let mut current = vertices.next().unwrap();
    std::iter::once(current).chain(
        std::iter::from_fn(move || {
            if let Some(next_vertex) = vertices.next() {
                let direction = Pos::new(
                    (next_vertex.x - current.x).signum(),
                    (next_vertex.y - current.y).signum(),
                );
                let mut current_edge = current;
                current = next_vertex;
                let mut done = false;
                Some(std::iter::from_fn(move || {
                    current_edge = current_edge + direction;
                    if done {
                        None
                    } else {
                        if current_edge == next_vertex {
                            done = true
                        }
                        Some(current_edge)
                    }
                }))
            } else {
                None
            }
        })
        .flatten(),
    )
}

fn find_bounds(input: &str) -> (Pos, Pos) {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .flat_map(|line| parse_vertices(line))
        .fold(
            (Pos::new(500, 0), Pos::new(500, 0)),
            |(top_left, bottom_right), item| {
                (
                    Pos::new(top_left.x.min(item.x), top_left.y.min(item.y)),
                    Pos::new(bottom_right.x.max(item.x), bottom_right.y.max(item.y)),
                )
            },
        )
}

pub fn part1(input: &str) -> u64 {
    Reservoir::from_input(input, false).simulate()
}

pub fn part2(input: &str) -> u64 {
    Reservoir::from_input(input, true).simulate()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_parse_path() {
        assert_eq!(
            parse_path("498,4 -> 498,6 -> 496,6").collect::<Vec<_>>(),
            [
                Pos::new(498, 4),
                Pos::new(498, 5),
                Pos::new(498, 6),
                Pos::new(497, 6),
                Pos::new(496, 6)
            ]
        );
    }

    #[test]
    fn test_find_bounds() {
        assert_eq!(
            find_bounds(TEST_INPUT),
            (Pos::new(494, 0), Pos::new(503, 9))
        )
    }

    #[test]
    fn test_from_input() {
        let reservoir = Reservoir::from_input(TEST_INPUT, false);
        for y in reservoir.top_left.y..=reservoir.bottom_right.y {
            println!();
            for x in reservoir.top_left.x..=reservoir.bottom_right.x {
                let pos = Pos::new(x, y);
                print!("{}", if reservoir.is_occupied(pos) { '#' } else { '.' });
            }
        }
        println!();
        // assert!(false);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 24);
        assert_eq!(part1(INPUT), 779);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 93);
        assert_eq!(part2(INPUT), 27426);
    }
}
