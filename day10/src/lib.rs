pub const INPUT: &str = include_str!("input.txt");

use std::{convert::identity, iter::repeat};

fn signal_strengths(input: &str) -> impl Iterator<Item = i32> + '_ {
    cycles(input).enumerate().map(|(i, x)| (i + 1) as i32 * x)
}

fn cycles(input: &str) -> impl Iterator<Item = i32> + '_ {
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

pub fn screen_chars(input: &str) -> impl Iterator<Item = char> + '_ {
    cycles(input)
        .map(|x| (x - 1)..=(x + 1))
        .enumerate()
        .flat_map(|(i, cursor)| {
            let index = i as i32 % 40;
            [
                Some(if cursor.contains(&index) { '#' } else { '.' }),
                if index == 39 { Some('\n') } else { None },
            ]
        })
        .filter_map(identity)
}

pub fn part2(input: &str) {
    screen_chars(input).for_each(|c| print!("{}", c));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const SMALL_PROGRAM: &str = "noop
addx 3
addx -5
";
    const TEST_SCREEN: &str = include_str!("test_screen.txt");
    const PART2_RESULT: &str = include_str!("part2_result.txt");

    #[test]
    fn test_x_values() {
        assert_eq!(cycles(SMALL_PROGRAM).count(), 5);
        assert_eq!(cycles(SMALL_PROGRAM).collect::<Vec<i32>>(), [1, 1, 1, 4, 4]);
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

    #[test]
    fn test_screen_chars() {
        let mut result = String::new();
        screen_chars(TEST_INPUT).for_each(|c| result.push(c));
        assert_eq!(
            result.trim().lines().collect::<Vec<_>>(),
            TEST_SCREEN.trim().lines().collect::<Vec<_>>()
        );
        let mut result = String::new();
        screen_chars(INPUT).for_each(|c| result.push(c));
        assert_eq!(
            result.trim().lines().collect::<Vec<_>>(),
            PART2_RESULT.trim().lines().collect::<Vec<_>>()
        );
    }
}
