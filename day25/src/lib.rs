pub const INPUT: &str = include_str!("input.txt");

fn to_dec(snafu: &str) -> i64 {
    let mut result = 0;
    for &b in snafu.as_bytes().iter() {
        result = result * 5
            + match b {
                b'0' => 0,
                b'1' => 1,
                b'2' => 2,
                b'-' => -1,
                b'=' => -2,
                _ => panic!("invalid snafu numebr"),
            }
    }
    result
}

fn to_snafu(val: i64) -> String {
    fn convert(val: i64, result: &mut Vec<u8>) {
        let quot = val / 5;
        let rem = (val % 5) as u8;
        let (v, carry) = match rem {
            0 | 1 | 2 => (rem + b'0', 0),
            3 => (b'=', 1),
            4 => (b'-', 1),
            _ => panic!(),
        };
        result.push(v);
        let quot = quot + carry;
        if quot > 0 {
            convert(quot, result)
        }
    }
    let mut r = Vec::with_capacity(20);
    convert(val, &mut r);
    r.reverse();
    String::from_utf8(r).unwrap()
}

pub fn part1(input: &str) -> String {
    to_snafu(
        input
            .lines()
            .filter(|l| !l.is_empty())
            .map(to_dec)
            .sum::<i64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    const TEST_VALUES: [(i64, &str); 15] = [
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
    ];

    #[test]
    fn test_to_dec() {
        for (dec, snafu) in TEST_VALUES {
            assert_eq!(to_dec(snafu), dec);
        }
    }

    #[test]
    fn test_to_snafu() {
        for (dec, snafu) in TEST_VALUES {
            assert_eq!(to_snafu(dec), snafu);
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "2=-1=0");
        assert_eq!(part1(INPUT), "2-==10--=-0101==1201");
    }
}
