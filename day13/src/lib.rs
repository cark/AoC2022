pub const INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> usize {
    let mut lines = input.lines().filter(|line| !line.is_empty());
    std::iter::from_fn(move || lines.next().map(|line1| (line1, lines.next().unwrap())))
        .enumerate()
        .filter_map(|(i, (l, r))| is_ordered(&mut tokenize(l), &mut tokenize(r)).then_some(i + 1))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut packets = input
        .lines()
        .filter(|line| !line.is_empty())
        .chain(["[[2]]", "[[6]]"])
        .map(|line| Packet {
            s: line,
            tokens: tokenize(line).collect(),
        })
        .collect::<Vec<_>>();
    packets.sort();
    packets
        .into_iter()
        .enumerate()
        .filter_map(|(i, packet)| (packet.s == "[[2]]" || packet.s == "[[6]]").then_some(i + 1))
        .product()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    StartList,
    EndList,
    Value(u8),
}

fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    use Token::*;
    let mut start = 0;
    std::iter::from_fn(move || {
        loop {
            if start == input.len() {
                break;
            }
            match input.as_bytes()[start] {
                b'[' => {
                    start += 1;
                    return Some(StartList);
                }
                b']' => {
                    start += 1;
                    return Some(EndList);
                }
                b',' => {
                    start += 1;
                }
                _digit => {
                    let len = input[start..]
                        .chars()
                        .take_while(char::is_ascii_digit)
                        .count();
                    let result = Some(Value(input[start..start + len].parse().unwrap()));
                    start += len;
                    return result;
                }
            }
        }
        None
    })
}

fn is_ordered<'a>(
    left: &mut (dyn Iterator<Item = Token> + 'a),
    right: &mut (dyn Iterator<Item = Token> + 'a),
) -> bool {
    use Token::*;
    loop {
        let pair = (left.next(), right.next());
        match pair {
            (None, None) => return true,
            (None, _) => unreachable!(),
            (_, None) => unreachable!(),
            (Some(l), Some(r)) => match (l, r) {
                (StartList, r) => match r {
                    StartList => {}
                    EndList => return false,
                    Value(r) => {
                        return is_ordered(left, &mut [Value(r), EndList].into_iter().chain(right))
                    }
                },
                (EndList, r) => match r {
                    StartList => return true,
                    EndList => {}
                    Value(_) => return true,
                },
                (Value(l), r) => match r {
                    StartList => {
                        return is_ordered(&mut [Value(l), EndList].into_iter().chain(left), right)
                    }
                    EndList => return false,
                    Value(r) => {
                        if l < r {
                            return true;
                        } else if r < l {
                            return false;
                        }
                    }
                },
            },
        }
    }
}

#[derive(Debug)]
struct Packet<'a> {
    s: &'a str,
    tokens: Vec<Token>,
}

impl PartialOrd for Packet<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if is_ordered(
            &mut self.tokens.iter().copied(),
            &mut other.tokens.iter().copied(),
        ) {
            core::cmp::Ordering::Less
        } else {
            core::cmp::Ordering::Greater
        }
    }
}

impl PartialEq for Packet<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.s == other.s
    }
}

impl Eq for Packet<'_> {
    fn assert_receiver_is_total_eq(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_tokenize() {
        use Token::*;
        let packet = "[10,[4,4],2]";
        assert_eq!(
            tokenize(packet).collect::<Vec<_>>(),
            [
                StartList,
                Value(10),
                StartList,
                Value(4),
                Value(4),
                EndList,
                Value(2),
                EndList
            ]
        );
    }

    #[test]
    fn test_is_ordered() {
        assert!(is_ordered(
            &mut tokenize("[1,1,3,1,1]"),
            &mut tokenize("[1,1,5,1,1]")
        ));
        assert!(is_ordered(
            &mut tokenize("[[1],[2,3,4]]"),
            &mut tokenize("[[1],4]")
        ));
        assert!(!is_ordered(
            &mut tokenize("[9]"),
            &mut tokenize("[[8,7,6]]")
        ));
        assert!(is_ordered(
            &mut tokenize("[[4,4],4,4]"),
            &mut tokenize("[[4,4],4,4,4]")
        ));
        assert!(!is_ordered(
            &mut tokenize("[7,7,7,7]"),
            &mut tokenize("[7,7,7]")
        ));
        assert!(is_ordered(&mut tokenize("[]"), &mut tokenize("[3]")));
        assert!(!is_ordered(&mut tokenize("[[[]]]"), &mut tokenize("[[]]")));
        assert!(!is_ordered(
            &mut tokenize("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
            &mut tokenize("[1,[2,[3,[4,[5,6,0]]]],8,9]")
        ));
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
        assert_eq!(part1(INPUT), 5588);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 140);
        assert_eq!(part2(INPUT), 23958);
    }
}