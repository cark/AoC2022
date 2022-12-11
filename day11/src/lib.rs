use std::cmp::Reverse;

pub const INPUT: &str = include_str!("input.txt");

struct Monkey {
    op: Op,
    test: Test,
    items: Vec<u64>,
    inspection_count: u64,
}

impl Monkey {
    fn parse<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Option<Self> {
        if let Some(line) = lines.next() {
            let _monkey_id = (line.as_bytes()[7] - b'0') as usize;
            let items = lines
                .next()
                .unwrap()
                .split(':')
                .skip(1)
                .next()
                .unwrap()
                .split(',')
                .map(|worry| worry.trim().parse().unwrap())
                .collect();
            let op = Op::parse(lines.next().unwrap().split('=').skip(1).next().unwrap());
            let test = Test::parse(lines);
            Some(Self {
                op,
                test,
                items,
                inspection_count: 0,
            })
        } else {
            None
        }
    }

    fn moves(&mut self, keep_calm: bool) -> impl Iterator<Item = (u64, usize)> + '_ {
        let inspection_count = &mut self.inspection_count;
        let op = &self.op;
        let test = &self.test;
        self.items.iter().map(move |worry| {
            *inspection_count += 1;
            let mut worry = op.apply(*worry);
            if keep_calm {
                worry = worry / 3
            }
            let next_monkey = test.test(worry);
            (worry, next_monkey)
        })
    }
}

struct Test {
    true_monkey: usize,
    false_monkey: usize,
    divisor: u64,
}

impl Test {
    fn parse<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Self {
        let divisor = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let true_monkey = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let false_monkey = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        Self {
            divisor,
            false_monkey,
            true_monkey,
        }
    }

    fn test(&self, worry: u64) -> usize {
        if worry % self.divisor == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }
}

enum Op {
    Mul(Value, Value),
    Add(Value, Value),
}

impl Op {
    fn parse(str: &str) -> Self {
        let mut tokens = str.split_whitespace();
        let left = Value::parse(tokens.next().unwrap());
        let op_str = tokens.next().unwrap();
        let right = Value::parse(tokens.next().unwrap());
        match op_str {
            "+" => Self::Add(left, right),
            "*" => Self::Mul(left, right),
            _ => unreachable!("Unknown op: {op_str}."),
        }
    }

    fn apply(&self, old: u64) -> u64 {
        match self {
            Op::Mul(l, r) => l.get(old) * r.get(old),
            Op::Add(l, r) => l.get(old) + r.get(old),
        }
    }
}

enum Value {
    Old,
    Literal(u64),
}

impl Value {
    fn parse(str: &str) -> Self {
        if str == "old" {
            Self::Old
        } else {
            Self::Literal(str.parse().unwrap())
        }
    }

    fn get(&self, old: u64) -> u64 {
        match self {
            Value::Old => old,
            Value::Literal(lit) => *lit,
        }
    }
}

struct KeepAwayGame {
    monkeys: Vec<Monkey>,
    max_worry: u64,
    moves: Vec<(u64, usize)>,
}

impl KeepAwayGame {
    fn parse(input: &str) -> Self {
        let mut monkeys = Vec::with_capacity(16);
        let mut lines = input.lines().filter(|l| !l.is_empty());
        while let Some(monkey) = Monkey::parse(&mut lines) {
            monkeys.push(monkey);
        }
        let max_worry = monkeys
            .iter()
            .map(|monkey| monkey.test.divisor)
            .product::<u64>();
        KeepAwayGame {
            monkeys,
            max_worry,
            moves: Vec::with_capacity(64),
        }
    }

    fn round(&mut self, keep_calm: bool) {
        for i in 0..self.monkeys.len() {
            let source_monkey = &mut self.monkeys[i];
            // gather moves
            self.moves.clear();
            for m in source_monkey.moves(keep_calm) {
                self.moves.push(m);
            }
            source_monkey.items.clear();
            //apply moves
            for (worry, monkey_index) in &self.moves {
                self.monkeys[*monkey_index]
                    .items
                    .push(*worry % self.max_worry);
            }
        }
    }

    fn run_rounds(&mut self, round_count: usize, keep_calm: bool) {
        for _i in 0..round_count {
            self.round(keep_calm)
        }
    }
}

fn solve(input: &str, rounds: usize, keep_calm: bool) -> u64 {
    let mut game = KeepAwayGame::parse(input);
    game.run_rounds(rounds, keep_calm);
    game.monkeys
        .sort_by_key(|monkey| Reverse(monkey.inspection_count));
    game.monkeys[0].inspection_count * game.monkeys[1].inspection_count
}

pub fn part1(input: &str) -> u64 {
    solve(input, 20, true)
}

pub fn part2(input: &str) -> u64 {
    solve(input, 10000, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_parsing() {
        let kag = KeepAwayGame::parse(TEST_INPUT);
        assert_eq!(kag.monkeys.len(), 4);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 10605);
        assert_eq!(part1(INPUT), 182293);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2713310158);
        assert_eq!(part2(INPUT), 54832778815);
    }
}
