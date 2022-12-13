pub const INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> usize {
    let mut lines = input.lines().filter(|line| !line.is_empty());
    std::iter::from_fn(move || lines.next().map(|line1| (line1, lines.next().unwrap())))
        .enumerate()
        .filter_map(|(i, (l, r))| is_ordered(&mut tokenize(l), &mut tokenize(r)).then_some(i + 1))
        .sum()
}

pub fn part2(input: &str) -> usize {
    fn index_of_virtual_packet(input: &str, virtual_packet: &str) -> usize {
        1 + input
            .lines()
            .filter(|line| !line.is_empty())
            .filter(|line| is_ordered(&mut tokenize(line), &mut tokenize(virtual_packet)))
            .count()
    }

    let index2 = index_of_virtual_packet(input, "[[2]]");
    let index6 = index_of_virtual_packet(input, "[[6]]") + 1;
    index2 * index6
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    StartList,
    EndList,
    Value(u8),
}

fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    use Token::*;
    let mut bytes = input.as_bytes().iter().peekable();
    std::iter::from_fn(move || loop {
        if let Some(&byte) = bytes.next() {
            match byte {
                b'[' => return Some(StartList),
                b']' => return Some(EndList),
                b',' => {}
                digit => {
                    let mut value = digit - b'0';
                    while let Some(&byte) = bytes.peek() {
                        if (b'0'..=b'9').contains(byte) {
                            bytes.next();
                            value = value * 10 + byte - b'0';
                        } else {
                            break;
                        }
                    }
                    return Some(Value(value));
                }
            }
        } else {
            return None;
        }
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
            (None, None) => return true,
            (None, _) => unreachable!(),
            (_, None) => unreachable!(),
        }
    }
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
