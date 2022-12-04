use std::ops::RangeInclusive;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let setup_time = std::time::Instant::now();
    let part1 = part1(INPUT);
    let part1_dur = setup_time.elapsed().as_micros();
    println!("Part1 : {} in {} µs", part1, part1_dur);

    let setup_time = std::time::Instant::now();
    let part2 = part2(INPUT);
    let part2_dur = setup_time.elapsed().as_micros();
    println!("Part2 : {} in {} µs", part2, part2_dur);
}

fn part1(input: &str) -> usize {
    solve(input, contains)
}

fn part2(input: &str) -> usize {
    solve(input, overlaps)
}

fn solve(
    input: &str,
    filter: impl Fn(&RangeInclusive<usize>, &RangeInclusive<usize>) -> bool,
) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut ranges = line.split(',');
            (
                make_range(ranges.next().unwrap()),
                make_range(ranges.next().unwrap()),
            )
        })
        .filter(|(r1, r2)| filter(r1, r2))
        .count()
}

fn make_range(txt: &str) -> RangeInclusive<usize> {
    let mut bounds = txt.split('-');
    bounds.next().unwrap().parse::<usize>().unwrap()
        ..=(bounds.next().unwrap().parse::<usize>().unwrap())
}

fn contains(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    (r1.start() <= r2.start() && r1.end() >= r2.end())
        || (r2.start() <= r1.start() && r2.end() >= r1.end())
}

fn overlaps(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    (r1.start() <= r2.start() && r1.end() >= r2.start())
        || (r2.start() <= r1.start() && r2.end() >= r1.start())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 2);
        assert_eq!(part1(INPUT), 441);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4);
        assert_eq!(part2(INPUT), 861);
    }
}
