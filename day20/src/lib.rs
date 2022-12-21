// we should go back to array based solution now that
// we know the bug was about modulos

pub const INPUT: &str = include_str!("input.txt");
const KEY: i64 = 811589153;

type Number = i64;
type LinkId = usize;
type NumberId = usize;

struct State {
    numbers: Vec<Number>,
    links: Vec<Link>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Link {
    left: LinkId,
    right: LinkId,
    number: Number,
    index: LinkId,
}

impl State {
    fn new(numbers: Vec<Number>) -> Self {
        let mut links = vec![];
        for i in 0..numbers.len() {
            let link = Link {
                left: pos_mod(i as Number - 1, numbers.len()),
                right: (i + 1) % numbers.len(),
                number: numbers[i],
                index: i,
            };
            links.push(link);
        }
        Self { numbers, links }
    }

    fn parse(input: &str) -> Self {
        let numbers = input
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();
        Self::new(numbers)
    }

    fn move_number(&mut self, num_index: NumberId, key: Number) {
        let len = self.links.len() as Number - 1;
        let link = self.links[num_index];
        let number = link.number * key;
        let moves = number.rem_euclid(len);
        let mut index = link.index;
        if moves == 0 {
            return;
        }
        //println!("{moves}");
        for _ in 0..moves {
            index = self.right_index(index);
        }
        self.remove_link(link.index);
        self.insert_link(link.index, index, self.right_index(index));
    }

    fn remove_link(&mut self, index: NumberId) {
        let (left, right) = {
            let link = &self.links[index];
            (link.left, link.right)
        };
        self.links[left].right = right;
        self.links[right].left = left;
    }

    fn insert_link(&mut self, index: NumberId, left: NumberId, right: NumberId) {
        self.links[left].right = index;
        self.links[right].left = index;
        let link = &mut self.links[index];
        link.left = left;
        link.right = right;
    }

    #[cfg(test)]
    fn moved_numbers(&self, start_id: LinkId) -> Vec<Number> {
        let mut result = Vec::with_capacity(self.numbers.len());
        let mut id = start_id;
        loop {
            let link = &self.links[id];
            result.push(link.number);
            id = link.right;
            if id == start_id {
                break;
            }
        }
        result
    }

    fn right_index(&self, index: LinkId) -> LinkId {
        self.links[index].right
    }

    fn index_of(&self, number: Number) -> LinkId {
        let mut index = 0;
        while self.links[index].number != number {
            index = self.right_index(index);
        }
        index
    }

    fn mix(&mut self, iterations: usize, key: Number) -> [LinkId; 3] {
        for _ in 0..iterations {
            for i in 0..self.numbers.len() {
                self.move_number(i, key);
            }
        }
        let mut result = [0; 3];
        let zero_index = self.index_of(0);
        let mut index = zero_index;
        for i in 0..3 {
            for _ in 0..1000 {
                index = self.right_index(index);
            }
            result[i] = index;
        }
        result
    }
}

pub fn part1(input: &str) -> i64 {
    let mut state = State::parse(input);
    state
        .mix(1, 1)
        .iter()
        .map(|&index| state.links[index].number)
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let mut state = State::parse(input);
    state
        .mix(10, KEY)
        .iter()
        .map(|&index| state.links[index].number * KEY)
        .sum()
}

fn pos_mod(val: i64, div: usize) -> usize {
    let div = div as i64;
    ((val % div + div) % div) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    fn cycle_equals(v1: &[i64], v2: &[i64]) -> bool {
        for offset in 0..v1.len() {
            if (0..v1.len()).all(|i| v1[(i + offset) % v1.len()] == v2[i]) {
                return true;
            }
        }
        false
    }

    #[test]
    fn test_modulo() {
        assert_eq!(0i64 % 7, 0);
        // assert_eq!(-4i64 % 4, -1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3);
        assert_eq!(part1(INPUT), 7713);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1623178306);
        assert_eq!(part2(INPUT), 1664569352803);
    }

    #[test]
    fn test_moves() {
        let mut state = State::new(vec![4, 5, 6, 1, 7, 8, 9]);
        state.move_number(3, 1);
        assert_eq!(state.moved_numbers(0), [4, 5, 6, 7, 1, 8, 9]);
        let mut state = State::new(vec![4, -2, 5, 6, 7, 8, 9]);
        state.move_number(1, 1);
        assert_eq!(state.moved_numbers(0), [4, 5, 6, 7, 8, -2, 9]);
        let mut state = State::parse(TEST_INPUT);
        std::iter::once(state.moved_numbers(0))
            .chain((0..7).map(move |i| {
                state.move_number(i, 1);
                let result = state.moved_numbers(0);
                //println!("{:?}", result);
                result
            }))
            .zip([
                [1, 2, -3, 3, -2, 0, 4],
                [2, 1, -3, 3, -2, 0, 4],
                [1, -3, 2, 3, -2, 0, 4],
                [1, 2, 3, -2, -3, 0, 4],
                [1, 2, -2, -3, 0, 3, 4],
                [1, 2, -3, 0, 3, 4, -2],
                [1, 2, -3, 0, 3, 4, -2],
                [1, 2, -3, 4, 0, 3, -2],
            ])
            //.for_each(|(left, right)| assert_eq!(&left, &right));
            .for_each(|(left, right)| assert!(cycle_equals(&left, &right)));
        //assert!(false);
    }

    #[test]
    fn test_moved_numbers() {
        let state = State::new(vec![4, 5, 6, 1, 7, 8, 9]);
        assert_eq!(state.moved_numbers(0), [4, 5, 6, 1, 7, 8, 9]);
    }

    #[test]
    fn test_remove_link() {
        let mut state = State::new(vec![0, 1, 2]);
        state.remove_link(1);
        // state.remove_link(state.links[1]);
        assert_eq!(state.moved_numbers(0), [0, 2]);
    }

    #[test]
    fn test_new() {
        let state = State::new(vec![4, 5, 6, 1, 7, 8, 9]);
        assert_eq!(
            state.links[0],
            Link {
                left: 6,
                right: 1,
                number: 4,
                index: 0,
            }
        );
        assert_eq!(
            state.links[6],
            Link {
                left: 5,
                right: 0,
                number: 9,
                index: 6,
            }
        );
        assert_eq!(
            state.links[3],
            Link {
                left: 2,
                right: 4,
                number: 1,
                index: 3
            }
        );
    }
}
