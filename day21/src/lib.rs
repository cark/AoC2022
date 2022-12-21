pub const INPUT: &str = include_str!("input.txt");

use std::collections::HashMap;

type MonkeyId = usize;
type MonkeyPair = (MonkeyId, MonkeyId);

struct Monkey<'a> {
    //    parent: Option<MonkeyId>,
    value: Value,
    name: &'a str,
}

#[derive(Debug)]
enum Value {
    Literal(i64),
    Op(OpType, MonkeyPair),
    //    X,
}

#[derive(Debug)]
enum OpType {
    Sum,
    Sub,
    Mul,
    Div,
    //    Eq,
}

enum SolveResult {
    Literal(i64),
    XAndOps(i64, i64),
}

struct Troop<'a> {
    monkeys: Vec<Option<Monkey<'a>>>,
    name_to_id: HashMap<&'a str, MonkeyId>,
}

impl<'a> Troop<'a> {
    fn parse(input: &'a str) -> Self {
        use OpType::*;
        use Value::*;
        input.lines().filter(|l| !l.is_empty()).fold(
            Self {
                monkeys: vec![],
                name_to_id: HashMap::new(),
            },
            |mut result, l| {
                let mut part = l.split(": ");
                let name = part.next().unwrap();
                let mut tokens = part.next().unwrap().split_whitespace();
                let first = tokens.next().unwrap();
                let id = result.name_id(name);
                match first.parse::<i64>() {
                    Ok(number) => result.set_monkey(
                        id,
                        Monkey {
                            //parent: None,
                            value: Literal(number),
                            name,
                        },
                    ),
                    Err(_) => {
                        let first = result.name_id(first);
                        let sign = tokens.next().unwrap();
                        let second = result.name_id(tokens.next().unwrap());
                        result.set_monkey(
                            id,
                            Monkey {
                                //parent: None,
                                name,
                                value: match sign {
                                    "+" => Op(Sum, (first, second)),
                                    "-" => Op(Sub, (first, second)),
                                    "*" => Op(Mul, (first, second)),
                                    "/" => Op(Div, (first, second)),
                                    _ => unreachable!(),
                                },
                            },
                        );
                    }
                }
                result
            },
        )
    }

    fn name_id<'b>(&'b mut self, name: &'a str) -> MonkeyId {
        if let Some(&id) = self.name_to_id.get(name) {
            id
        } else {
            let result = self.monkeys.len();
            self.monkeys.push(None);
            self.name_to_id.insert(name, result);
            result
        }
    }

    fn set_monkey<'b>(&'b mut self, id: MonkeyId, monkey: Monkey<'a>) {
        self.monkeys[id] = Some(monkey);
    }

    fn calc(&self, monkey_id: MonkeyId) -> i64 {
        use Value::*;
        let Some(monkey) = &self.monkeys[monkey_id] else {panic!("Monkey {monkey_id} does not exist")};
        match &monkey.value {
            Literal(l) => *l,
            Op(op_type, (l, r)) => apply_op(op_type, (self.calc(*l), self.calc(*r))),
        }
    }

    fn solve(&self, monkey_id: MonkeyId) -> SolveResult {
        use OpType::*;
        use SolveResult::*;
        let monkey = self.monkeys[monkey_id].as_ref().unwrap();
        if monkey.name == "humn" {
            XAndOps(1, 0)
        } else if monkey.name == "root" {
            let Value::Op(_, (left, right)) = monkey.value else { panic!() };
            match (self.solve(left), self.solve(right)) {
                (Literal(l), XAndOps(a, b)) | (XAndOps(a, b), Literal(l)) => {
                    println!("{a}, {b}");
                    Literal((l - b) / a)
                }
                _ => unreachable!(),
            }
        } else {
            match &monkey.value {
                Value::Literal(v) => Literal(*v),
                Value::Op(op_type, (left_monkey, right_monkey)) => {
                    let left = self.solve(*left_monkey);
                    let right = self.solve(*right_monkey);
                    match (left, right) {
                        (Literal(a), Literal(b)) => Literal(apply_op(&op_type, (a, b))),
                        (Literal(l), XAndOps(a, b)) | (XAndOps(a, b), Literal(l)) => {
                            println!("a: {a}, b: {b}, op: {op_type:?}, l: {l}");
                            match op_type {
                                Sum | Sub => XAndOps(a, apply_op(op_type, (b, l))),
                                Mul | Div => {
                                    XAndOps(apply_op(op_type, (a, l)), apply_op(op_type, (b, l)))
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}

fn apply_op(op: &OpType, pair: (i64, i64)) -> i64 {
    use OpType::*;
    match op {
        Sum => pair.0 + pair.1,
        Sub => pair.0 - pair.1,
        Mul => pair.0 * pair.1,
        Div => pair.0 / pair.1,
    }
}

pub fn part1(input: &str) -> i64 {
    let mut troop = Troop::parse(input);
    let root_id = troop.name_id("root");
    troop.calc(root_id)
}

pub fn part2(input: &str) -> i64 {
    let mut troop = Troop::parse(input);
    let root_id = troop.name_id("root");
    let SolveResult::Literal(result) = troop.solve(root_id) else {unreachable!()};
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 152);
        assert_eq!(part1(INPUT), 87457751482938);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 301);
        //assert_eq!(part2(INPUT), 87457751482938);
    }
}
