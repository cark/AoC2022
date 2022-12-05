const INPUT: &str = include_str!("input.txt");

fn main() {
    let setup_time = std::time::Instant::now();
    let part1 = part1(INPUT);
    let part1_dur = setup_time.elapsed().as_nanos();
    println!("Part1 : {} in {} ns", part1, part1_dur);

    let setup_time = std::time::Instant::now();
    let part2 = part2(INPUT);
    let part2_dur = setup_time.elapsed().as_nanos();
    println!("Part2 : {} in {} ns", part2, part2_dur);
}

fn part1(input: &str) -> String {
    solve::<Model9000>(input)
}

fn part2(input: &str) -> String {
    solve::<Model9001>(input)
}

fn solve<C: Crane>(input: &str) -> String {
    let mut input = parse(input);
    exec_moves::<C>(&mut input);
    collect_top_crates(&input)
}

fn exec_moves<C: Crane>(input: &mut Input) {
    for m in input.moves.iter() {
        let stacks = &mut input.stacks;
        let mut from_stack = std::mem::take(&mut stacks[m.from - 1]);
        let mut to_stack = std::mem::take(&mut stacks[m.to - 1]);
        C::apply_move(m.qty, &mut from_stack, &mut to_stack);
        stacks[m.from - 1] = from_stack;
        stacks[m.to - 1] = to_stack;
    }
}

struct Input {
    stacks: Stacks,
    moves: Vec<Move>,
}

type Stacks = Vec<Stack>;
type Stack = Vec<u8>;

#[derive(Debug, PartialEq)]
struct Move {
    from: usize,
    to: usize,
    qty: u8,
}

trait Crane {
    fn apply_move(crate_count: u8, from_stack: &mut Stack, to_stack: &mut Stack);
}

struct Model9000;

impl Crane for Model9000 {
    fn apply_move(crate_count: u8, from_stack: &mut Stack, to_stack: &mut Stack) {
        for _ in 0..crate_count {
            to_stack.push(from_stack.pop().unwrap())
        }
    }
}

struct Model9001;

impl Crane for Model9001 {
    fn apply_move(crate_count: u8, from_stack: &mut Stack, to_stack: &mut Stack) {
        let from_index = from_stack.len() - crate_count as usize;
        to_stack.extend_from_slice(&from_stack[from_index..]);
        from_stack.truncate(from_index);
    }
}

fn collect_top_crates(input: &Input) -> String {
    // Safety: I built the damn thing myself ! (everything an ascii char)
    // I'd still unwrap it anyways
    unsafe {
        String::from_utf8_unchecked(
            input
                .stacks
                .iter()
                .map(|stack| stack.last().unwrap())
                .copied()
                .collect::<Vec<u8>>(),
        )
    }
}

fn parse(input: &str) -> Input {
    let (stacks, rest) = parse_stacks(input);
    let moves = parse_moves(rest);
    Input { stacks, moves }
}

fn parse_stacks(mut input: &str) -> (Stacks, &str) {
    let mut stacks: Vec<Vec<u8>> = vec![];
    loop {
        let (line, rest) = input.split_once('\n').unwrap();
        input = rest;
        if line.trim().is_empty() {
            for stack in stacks.iter_mut() {
                stack.reverse()
            }
            return (stacks, rest);
        } else {
            let char = line.as_bytes()[1];
            if char >= b'1' && char <= b'9' {
                continue;
            } else {
                for i in 0.. {
                    if let Some(&name) = line.as_bytes().get(i * 4 + 1) {
                        if name >= b'A' && name <= b'Z' {
                            if stacks.len() <= i {
                                stacks.resize_with(i + 1, Default::default);
                            }
                            stacks[i].push(name);
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut tokens = line.split_whitespace();
            Move {
                qty: tokens.nth(1).unwrap().parse::<u8>().unwrap(),
                from: tokens.nth(1).unwrap().parse::<usize>().unwrap(),
                to: tokens.nth(1).unwrap().parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<Move>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_parsing() {
        let input = parse(TEST_INPUT);
        assert_eq!(input.stacks.len(), 3);
        assert_eq!(input.stacks[0], [b'Z', b'N']);
        assert_eq!(input.stacks[1], [b'M', b'C', b'D']);
        assert_eq!(input.stacks[2], [b'P']);
        assert_eq!(
            input.moves,
            [
                Move {
                    qty: 1,
                    from: 2,
                    to: 1
                },
                Move {
                    qty: 3,
                    from: 1,
                    to: 3
                },
                Move {
                    qty: 2,
                    from: 2,
                    to: 1
                },
                Move {
                    qty: 1,
                    from: 1,
                    to: 2
                },
            ]
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "CMZ");
        assert_eq!(part1(INPUT), "CVCWCRTVQ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "MCD");
        assert_eq!(part2(INPUT), "CNSCZWLVT");
    }
}
