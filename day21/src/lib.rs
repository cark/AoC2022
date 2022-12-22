pub const INPUT: &str = include_str!("input.txt");

use std::collections::HashMap;

type MonkeyId = usize;
type Number = i64;
type MonkeyPair = (MonkeyId, MonkeyId);

#[derive(Debug, Copy, Clone)]
enum Monkey {
    Literal(Number),
    Op(MathOp, MonkeyPair),
    Eq(MonkeyPair),
    Variable,
}

#[derive(Debug, Copy, Clone)]
enum MathOp {
    Sum,
    Sub,
    Mul,
    Div,
}

impl MathOp {
    fn apply(self, left: Number, right: Number) -> Number {
        use MathOp::*;
        match self {
            Sum => left + right,
            Sub => left - right,
            Mul => left * right,
            Div => left / right,
        }
    }

    fn inverse(self) -> MathOp {
        use MathOp::*;
        match self {
            Sum => Sub,
            Sub => Sum,
            Mul => Div,
            Div => Mul,
        }
    }
}

enum EvalResult {
    Literal(Number),
    VarResult(Vec<ResultPair>),
}

enum ResultPair {
    VarLeft(MathOp, Number),
    VarRight(MathOp, Number),
}

struct Troop<'a> {
    monkeys: Vec<Option<Monkey>>,
    name_to_id: HashMap<&'a str, MonkeyId>,
}

impl<'a> Troop<'a> {
    fn parse(input: &'a str) -> Self {
        use MathOp::*;
        use Monkey::*;
        input.lines().filter(|l| !l.is_empty()).fold(
            Self {
                monkeys: vec![],
                name_to_id: Default::default(),
            },
            |mut result, l| {
                let mut part = l.split(": ");
                let name = part.next().unwrap();
                let mut tokens = part.next().unwrap().split_whitespace();
                let first = tokens.next().unwrap();
                let id = result.name_id(name);
                match first.parse::<i64>() {
                    Ok(number) => result.set_monkey(id, Literal(number)),
                    Err(_) => {
                        let first = result.name_id(first);
                        let sign = tokens.next().unwrap();
                        let second = result.name_id(tokens.next().unwrap());
                        result.set_monkey(
                            id,
                            match sign {
                                "+" => Op(Sum, (first, second)),
                                "-" => Op(Sub, (first, second)),
                                "*" => Op(Mul, (first, second)),
                                "/" => Op(Div, (first, second)),
                                _ => unreachable!(),
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

    fn set_monkey<'b>(&'b mut self, id: MonkeyId, monkey: Monkey) {
        self.monkeys[id] = Some(monkey);
    }

    fn eval(&self, id: MonkeyId) -> EvalResult {
        use EvalResult::*;
        use MathOp::*;
        use ResultPair::*;
        match self.monkeys[id].unwrap() {
            Monkey::Literal(n) => Literal(n),
            Monkey::Op(op, (l, r)) => match (self.eval(l), self.eval(r)) {
                (Literal(left), Literal(right)) => Literal(op.apply(left, right)),
                (Literal(left), VarResult(mut right)) => {
                    right.push(VarRight(op, left));
                    VarResult(right)
                }
                (VarResult(mut left), Literal(right)) => {
                    left.push(VarLeft(op, right));
                    VarResult(left)
                }
                _ => unreachable!(),
            },
            Monkey::Variable => VarResult(Vec::with_capacity(self.monkeys.len())),
            Monkey::Eq((l, r)) => match (self.eval(l), self.eval(r)) {
                (Literal(l), VarResult(mut pairs)) | (VarResult(mut pairs), Literal(l)) => {
                    let mut result = l;
                    while let Some(pair) = pairs.pop() {
                        result = match pair {
                            VarLeft(op, num) => op.inverse().apply(result, num),
                            VarRight(op, num) => match op {
                                Sum | Mul => op.inverse().apply(result, num),
                                Sub | Div => op.apply(num, result),
                            },
                        };
                    }
                    Literal(result)
                }
                _ => unreachable!(),
            },
        }
    }
}

pub fn part1(input: &str) -> Number {
    use EvalResult::*;
    let troop = Troop::parse(input);
    let root = troop.name_to_id["root"];
    let Literal(result) = troop.eval(root) else { unreachable!() };
    result
}

pub fn part2(input: &str) -> Number {
    use EvalResult::*;
    let mut troop = Troop::parse(input);
    let root = troop.name_to_id["root"];
    let human = troop.name_to_id["humn"];
    let r = troop.monkeys[root].as_mut().unwrap();
    let Monkey::Op(_, pair) = *r else { unreachable!()} ;
    *r = Monkey::Eq(pair);
    let r = troop.monkeys[human].as_mut().unwrap();
    *r = Monkey::Variable;
    let Literal(result) = troop.eval(root) else { unreachable!() };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT2: &str = include_str!("test_input2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 152);
        assert_eq!(part1(INPUT), 87457751482938);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT2), 19);
        assert_eq!(part2(TEST_INPUT), 301);
        assert_eq!(part2(INPUT), 3221245824363);
    }
}
