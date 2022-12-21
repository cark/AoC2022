pub const INPUT: &str = include_str!("input.txt");

use std::collections::HashMap;

type MonkeyId = usize;

enum Monkey {
    Literal(i64),
    Sum(MonkeyId, MonkeyId),
    Sub(MonkeyId, MonkeyId),
    Mul(MonkeyId, MonkeyId),
    Div(MonkeyId, MonkeyId),
}

struct Troop<'a> {
    monkeys: Vec<Option<Monkey>>,
    name_to_id: HashMap<&'a str, MonkeyId>,
}

impl<'a> Troop<'a> {
    fn parse(input: &'a str) -> Self {
        use Monkey::*;
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
                    Ok(number) => result.set_monkey(id, Literal(number)),
                    Err(_) => {
                        let first = result.name_id(first);
                        let sign = tokens.next().unwrap();
                        let second = result.name_id(tokens.next().unwrap());
                        result.set_monkey(
                            id,
                            match sign {
                                "+" => Sum(first, second),
                                "-" => Sub(first, second),
                                "*" => Mul(first, second),
                                "/" => Div(first, second),
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

    fn solve(&self, monkey_id: MonkeyId) -> i64 {
        let Some(monkey) = &self.monkeys[monkey_id] else {panic!("Monkey {monkey_id} does not exist")};
        match monkey {
            Monkey::Literal(l) => *l,
            Monkey::Sum(l, r) => self.solve(*l) + self.solve(*r),
            Monkey::Sub(l, r) => self.solve(*l) - self.solve(*r),
            Monkey::Mul(l, r) => self.solve(*l) * self.solve(*r),
            Monkey::Div(l, r) => self.solve(*l) / self.solve(*r),
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let mut troop = Troop::parse(input);
    let root_id = troop.name_id("root");
    troop.solve(root_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 152);
    }
}
