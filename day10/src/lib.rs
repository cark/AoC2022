pub const INPUT: &str = include_str!("input.txt");

use std::iter::{repeat, repeat_with};

fn signal_strengths(input: &str) -> impl Iterator<Item = i32> + '_ {
    cycle_values(input)
        .enumerate()
        .map(|(i, x)| (i + 1) as i32 * x)
}

fn cycle_values(input: &str) -> impl Iterator<Item = i32> + '_ {
    let mut x: i32 = 1;
    input
        .lines()
        .filter(|l| !l.is_empty())
        .flat_map(move |line| {
            let mut tokens = line.split_whitespace();
            let instr = tokens.next().unwrap();
            match instr {
                "noop" => repeat(x).take(1),
                "addx" => {
                    let result = repeat(x).take(2);
                    let to_add = tokens.next().unwrap().parse::<i32>().unwrap();
                    x += to_add;
                    result
                }
                _ => unreachable!("{instr} op unknown."),
            }
        })
}

pub fn part1(input: &str) -> i32 {
    signal_strengths(input).skip(19).step_by(40).take(6).sum()
}

pub fn part2(input: &str) {
    let rows = repeat_with(|| (0..).take(40)).take(6);
    let mut cursors = cycle_values(input).map(|x| (x - 1)..=(x + 1));
    rows.for_each(move |index_iter| {
        index_iter
            .map(|index| {
                let cursor = cursors.next().unwrap();
                if cursor.contains(&index) {
                    '#'
                } else {
                    '.'
                }
            })
            .for_each(|c| print!("{}", c));
        println!()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const SMALL_PROGRAM: &str = "noop
addx 3
addx -5
";

    #[test]
    fn test_x_values() {
        assert_eq!(cycle_values(SMALL_PROGRAM).count(), 5);
        assert_eq!(
            cycle_values(SMALL_PROGRAM).collect::<Vec<i32>>(),
            [1, 1, 1, 4, 4]
        );
    }

    #[test]
    fn test_signal_strengths() {
        let strengths = signal_strengths(TEST_INPUT)
            .skip(19)
            .step_by(40)
            .collect::<Vec<_>>();
        assert_eq!(strengths, [420, 1140, 1800, 2940, 2880, 3960]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13140);
        assert_eq!(part1(INPUT), 11220);
    }
}
