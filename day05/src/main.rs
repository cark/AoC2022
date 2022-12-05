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

type Stack = Vec<u8>;
type Stacks = Vec<Stack>;

#[derive(Debug, PartialEq, Eq)]
struct Move {
    qty: u8,
    from: u8,
    to: u8,
}

struct Input {
    stacks: Stacks,
    moves: Vec<Move>,
}

fn parse(input: &str) -> Input {
    let (stacks, rest) = parse_stacks(input);
    let moves = parse_moves(rest);
    Input { stacks, moves }
}

//1 5 8
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
                for i in 0..9 {
                    if let Some(&name) = line.as_bytes().get(i * 4 + 1) {
                        if name >= b'A' && name <= b'Z' {
                            if stacks.len() <= i {
                                stacks.resize_with(i + 1, Default::default);
                            }
                            stacks[i].push(name);
                        }
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
                from: tokens.nth(1).unwrap().parse::<u8>().unwrap(),
                to: tokens.nth(1).unwrap().parse::<u8>().unwrap(),
            }
        })
        .collect::<Vec<Move>>()
}

fn exec_move_9000(m: &Move, stacks: &mut Stacks) {
    let from = m.from as usize - 1;
    let to = m.to as usize - 1;
    for _ in 0..m.qty {
        let item = stacks[from].pop().unwrap();
        stacks[to].push(item);
    }
}

fn exec_move_9001(m: &Move, stacks: &mut Stacks) {
    let from = m.from as usize - 1;
    let to = m.to as usize - 1;
    let copy_from = stacks[from].len() - m.qty as usize;
    let mut items = Vec::from(&stacks[from][copy_from..]);
    stacks[to].append(&mut items);
    stacks[from].truncate(copy_from);
}

fn exec_moves<F: Fn(&Move, &mut Stacks)>(input: &mut Input, f: F) {
    for m in input.moves.iter() {
        f(m, &mut input.stacks);
    }
}

fn collect_top_crates(input: &Input) -> String {
    // Safety: I built the damn thing myself ! (everything an ascii letter)
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

fn part1(input: &str) -> String {
    let mut input = parse(input);
    exec_moves(&mut input, exec_move_9000);
    collect_top_crates(&input)
}

fn part2(input: &str) -> String {
    let mut input = parse(input);
    exec_moves(&mut input, exec_move_9001);
    collect_top_crates(&input)
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
