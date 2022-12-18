use std::collections::VecDeque;

pub const INPUT: &str = include_str!("input.txt");

type Pos = (i32, i32, i32);

const NEIGHBORS: [Pos; 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

pub fn part1(input: &str) -> i32 {
    Field::parse(input).free_face_count()
}

pub fn part2(input: &str) -> i32 {
    Field::parse(input).exterior_face_count()
}

fn add_pos(pos1: Pos, pos2: Pos) -> Pos {
    (pos1.0 + pos2.0, pos1.1 + pos2.1, pos1.2 + pos2.2)
}

fn sub_pos(pos1: Pos, pos2: Pos) -> Pos {
    (pos1.0 - pos2.0, pos1.1 - pos2.1, pos1.2 - pos2.2)
}

fn neighbors(pos: &Pos) -> impl Iterator<Item = Pos> + '_ {
    NEIGHBORS.iter().map(move |&n| add_pos(n, *pos))
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item = Pos> + Clone + 'a {
    input.split_whitespace().filter(|s| !s.is_empty()).map(|s| {
        let mut coords = s.split(',').map(|s| s.parse::<i32>().unwrap());
        (
            coords.next().unwrap(),
            coords.next().unwrap(),
            coords.next().unwrap(),
        )
    })
}

#[derive(Clone, Copy)]
struct Bounds {
    low: Pos,
    lengths: Pos,
}

impl Bounds {
    fn new(positions: &[Pos]) -> Self {
        let (min, max) = positions.iter().fold(
            (
                (i32::MAX, i32::MAX, i32::MAX),
                (i32::MIN, i32::MIN, i32::MIN),
            ),
            |((min_x, min_y, min_z), (max_x, max_y, max_z)), pos| {
                (
                    (min_x.min(pos.0), min_y.min(pos.1), min_z.min(pos.2)),
                    (max_x.max(pos.0), max_y.max(pos.1), max_z.max(pos.2)),
                )
            },
        );
        let lengths = add_pos(sub_pos(max, min), (1, 1, 1));
        Self { low: min, lengths }
    }

    fn grow(self) -> Self {
        Self {
            low: sub_pos(self.low, (1, 1, 1)),
            lengths: add_pos(self.lengths, (2, 2, 2)),
        }
    }

    fn capacity(&self) -> usize {
        (self.lengths.0 * self.lengths.1 * self.lengths.2) as usize
    }

    fn index(&self, pos: Pos) -> usize {
        let pos = sub_pos(pos, self.low);
        (pos.0 + pos.1 * self.lengths.0 + pos.2 * self.lengths.0 * self.lengths.1) as usize
    }

    fn in_bounds(&self, pos: Pos) -> bool {
        (self.low.0..self.low.0 + self.lengths.0).contains(&pos.0)
            && (self.low.1..self.low.1 + self.lengths.1).contains(&pos.1)
            && (self.low.2..self.low.2 + self.lengths.2).contains(&pos.2)
    }
}

struct Field {
    bounds: Bounds,
    data: Vec<bool>,
    positions: Vec<Pos>,
}

impl Field {
    fn parse(input: &str) -> Self {
        let positions = parse(input).collect::<Vec<_>>();
        let bounds = Bounds::new(&positions).grow();
        let mut result = Self::new(bounds, Vec::with_capacity(bounds.capacity()));
        for pos in positions {
            result.set(pos);
        }
        result
    }

    fn new(bounds: Bounds, positions: Vec<Pos>) -> Self {
        let len = bounds.capacity();
        let mut data = Vec::with_capacity(len);
        data.resize(len, false);
        Self {
            bounds,
            data,
            positions,
        }
    }

    fn contains(&self, pos: Pos) -> bool {
        self.data[self.bounds.index(pos)]
    }

    fn set(&mut self, pos: Pos) {
        self.data[self.bounds.index(pos)] = true;
        self.positions.push(pos);
    }

    fn free_face_count(&self) -> i32 {
        self.positions
            .iter()
            .map(|&pos| neighbors(&pos).filter(|&n| !self.contains(n)).count() as i32)
            .sum()
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.positions.len()
    }

    fn enclosing_space(&self) -> Self {
        let mut result = Self::new(self.bounds, Vec::with_capacity(self.bounds.capacity()));
        let mut queue = VecDeque::new();
        queue.push_back(self.bounds.low);
        while let Some(pos) = queue.pop_front() {
            neighbors(&pos).for_each(|pos| {
                if self.bounds.in_bounds(pos) && !self.contains(pos) && !result.contains(pos) {
                    result.set(pos);
                    queue.push_back(pos);
                }
            })
        }
        result
    }

    fn exterior_face_count(&self) -> i32 {
        let enclosing = self.enclosing_space();
        self.positions
            .iter()
            .map(|pos| neighbors(&pos).filter(|&n| enclosing.contains(n)).count() as i32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const SHORT_INPUT: &str = "1,1,1 2,1,1";
    #[test]
    fn test_parsing() {
        assert_eq!(Field::parse(SHORT_INPUT).len(), 2);
        assert_eq!(Field::parse(TEST_INPUT).len(), 13);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SHORT_INPUT), 10);
        assert_eq!(part1(TEST_INPUT), 64);
        assert_eq!(part1(INPUT), 4314);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 58);
        assert_eq!(part2(INPUT), 2444);
    }
}
