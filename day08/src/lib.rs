pub const INPUT: &str = include_str!("input.txt");

struct Grid<'a> {
    width: usize,
    height: usize,
    stride: usize,
    data: &'a [u8],
    seen: Vec<bool>,
}

impl<'a> Grid<'a> {
    fn new(input: &'a str) -> Self {
        // We need the stride value, and that depends on the OS.
        // There is no end_of_line info in std, so we check that ourselves.
        enum State {
            Text,
            Eol,
        }
        use State::*;

        let mut max_len = 0;
        let mut max_stride = 0;
        let mut line_count = 0;
        let mut len = 0;
        let mut state = Text;
        for c in input.bytes() {
            match state {
                Text => match c {
                    13 | 10 => {
                        if max_len < len {
                            max_len = len;
                        }
                        line_count += 1;
                        state = Eol
                    }
                    _ => len += 1,
                },
                Eol => match c {
                    13 | 10 => len += 1,
                    _ => {
                        if max_stride < len {
                            max_stride = len;
                        }
                        len = 0;
                        state = Text
                    }
                },
            }
        }
        let mut seen = Vec::with_capacity(input.len());
        seen.resize(input.len(), false);
        Self {
            width: max_len,
            height: line_count,
            stride: max_stride + 1,
            data: input.as_bytes(),
            seen,
        }
    }

    fn pos_index(&self, pos: Pos) -> usize {
        (pos.0 + pos.1 * self.stride as i32) as usize
    }

    fn tree_height(&self, pos: Pos) -> u8 {
        self.data[self.pos_index(pos)] - b'0'
    }

    fn line_visible_trees(&mut self, line: impl Iterator<Item = Pos>) -> usize {
        let mut result = 0;
        let mut biggest = -1;
        for pos in line {
            let tree_height = self.tree_height(pos) as i32;
            let idx = self.pos_index(pos);
            if tree_height > biggest {
                biggest = tree_height;
                if !self.seen[idx] {
                    self.seen[idx] = true;
                    result += 1;
                }
            }
            if biggest >= 9 {
                break;
            }
        }
        result
    }

    fn lines_visible_trees(
        &mut self,
        lines: impl Iterator<Item = impl Iterator<Item = Pos>>,
    ) -> usize {
        lines.map(|line| self.line_visible_trees(line)).sum()
    }

    fn all_visible_trees(&mut self) -> usize {
        let left = lines_iterator((0, 0), (0, 1), self.height, (1, 0), self.width);
        let right = lines_iterator(
            (self.width as i32 - 1, 0),
            (0, 1),
            self.height,
            (-1, 0),
            self.width,
        );
        let top = lines_iterator((0, 0), (1, 0), self.width, (0, 1), self.height);
        let bottom = lines_iterator(
            (0, self.height as i32 - 1),
            (1, 0),
            self.width,
            (0, -1),
            self.height,
        );
        self.lines_visible_trees(left)
            + self.lines_visible_trees(right)
            + self.lines_visible_trees(top)
            + self.lines_visible_trees(bottom)
    }

    fn line_visible_from_tree(
        &self,
        mut line: impl Iterator<Item = Pos>,
        start_height: i32,
    ) -> usize {
        let mut result = 0;
        while let Some(pos) = line.next() {
            result += 1;
            if (self.tree_height(pos) as i32) >= start_height {
                break;
            }
        }
        result
    }

    fn part2(&self) -> usize {
        lines_iterator((0, 0), (0, 1), self.height, (1, 0), self.width)
            .flatten()
            .map(|pos| {
                let left = line_iterator(pos, (-1, 0), pos.0 as usize + 1);
                let right = line_iterator(pos, (1, 0), self.width - pos.0 as usize);
                let up = line_iterator(pos, (0, -1), pos.1 as usize + 1);
                let down = line_iterator(pos, (0, 1), self.height - pos.1 as usize);
                let start_height = self.tree_height(pos) as i32;
                let l = self.line_visible_from_tree(left.skip(1), start_height);
                let r = self.line_visible_from_tree(right.skip(1), start_height);
                let u = self.line_visible_from_tree(up.skip(1), start_height);
                let d = self.line_visible_from_tree(down.skip(1), start_height);
                l * r * u * d
            })
            .max()
            .unwrap()
    }
}

type Pos = (i32, i32);
type Direction = (i32, i32);

fn line_iterator(start_at: Pos, direction: Direction, len: usize) -> impl Iterator<Item = Pos> {
    (0..len as i32).map(move |n| (start_at.0 + direction.0 * n, start_at.1 + direction.1 * n))
}

fn lines_iterator(
    start_at: Pos,
    direction: Direction,
    len: usize,
    line_direction: Direction,
    line_len: usize,
) -> impl Iterator<Item = impl Iterator<Item = Pos>> {
    line_iterator(start_at, direction, len)
        .map(move |line_start| line_iterator(line_start, line_direction, line_len))
}

pub fn part1(input: &str) -> usize {
    Grid::new(input).all_visible_trees()
}

pub fn part2(input: &str) -> usize {
    Grid::new(input).part2()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_line_iterator() {
        assert_eq!(
            line_iterator((1, 0), (0, 1), 4).collect::<Vec<_>>(),
            [(1, 0), (1, 1), (1, 2), (1, 3)]
        );
    }

    #[test]
    fn test_lines_iterator() {
        assert_eq!(
            lines_iterator((0, 0), (0, 1), 2, (1, 0), 2)
                .map(|i| i.collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            [[(0, 0), (1, 0)], [(0, 1), (1, 1)]]
        );
    }

    #[test]
    fn test_new_grid() {
        let grid = Grid::new(TEST_INPUT);
        assert_eq!((grid.width, grid.height), (5, 5));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 21);
        assert_eq!(part1(INPUT), 1662);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 8);
        assert_eq!(part2(TEST_INPUT), 537600);
    }
}
