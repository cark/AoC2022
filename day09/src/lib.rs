pub const INPUT: &str = include_str!("input.txt");

type Pos = (i32, i32);

pub fn part1(input: &str) -> u64 {
    solve::<2>(input)
}

pub fn part2(input: &str) -> u64 {
    solve::<10>(input)
}

#[inline(always)]
fn set_bit(pos: Pos, slice: &mut [usize]) {
    let index = ((pos.0 + 512) + (pos.1 + 512) * 1024) as usize;
    let word = &mut slice[index >> 6];
    let shift = index & 0b111111;
    *word |= 1 << shift;
}

pub fn solve<const COUNT: usize>(input: &str) -> u64 {
    let mut knots: [Pos; COUNT] = [Pos::default(); COUNT];
    let indexes: [usize; COUNT] = std::array::from_fn(|i| i);
    let mut visited = Vec::with_capacity(1024 * 1024 / 64);
    visited.resize(1024 * 1024 / 64, 0usize);
    set_bit((0, 0), &mut visited);
    for direction in directions(input) {
        let head = &mut knots[0];
        *head = add_vec(*head, direction);
        for w in indexes.windows(2) {
            let head = knots[w[0]];
            let tail = &mut knots[w[1]];
            let old_tail = *tail;
            *tail = follows(*tail, head);
            if old_tail == *tail {
                break;
            }
        }
        let tail = knots[COUNT - 1];
        set_bit(tail, &mut visited);
    }
    visited.iter().map(|u| u.count_ones()).sum::<u32>() as u64
}

fn directions(input: &str) -> impl Iterator<Item = Pos> + '_ {
    input.lines().filter(|line| !line.is_empty()).flat_map(|s| {
        let mut tokens = s.split_whitespace();
        let direction = match tokens.next().unwrap() {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => unreachable!(),
        };
        let move_count = tokens.next().unwrap().parse::<usize>().unwrap();
        std::iter::repeat(direction).take(move_count)
    })
}

fn add_vec(vec1: Pos, vec2: Pos) -> Pos {
    (vec1.0 + vec2.0, vec1.1 + vec2.1)
}

fn follows(tail: Pos, head: Pos) -> Pos {
    let x_diff = head.0 - tail.0;
    let y_diff = head.1 - tail.1;
    let x_abs = x_diff.abs();
    let y_abs = y_diff.abs();
    if x_abs > 1 || y_abs > 1 {
        add_vec(tail, (x_diff.signum(), y_diff.signum()))
    } else {
        tail
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT2: &str = include_str!("test_input2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
        assert_eq!(part1(INPUT), 6391);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT2), 36);
        assert_eq!(part2(INPUT), 2593);
    }
}
