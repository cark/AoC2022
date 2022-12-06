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
    let (window_index, _) = input
        .as_bytes()
        .windows(4)
        .enumerate()
        .find(|(_, window)| is_start_marker(window))
        .unwrap();
    window_index + 4
}

fn part2(input: &str) -> usize {
    let (window_index, _) = input
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, window)| is_start_marker(window))
        .unwrap();
    window_index + 14
}

fn is_start_marker(window: &[u8]) -> bool {
    (0..window.len())
        .map(|i| (window[i], &window[i + 1..]))
        .all(|(item, slice)| !slice.contains(&item))
}

#[cfg(test)]
mod tests {
    use super::*;

    // const TEST_INPUT: &str = include_str!("test_input.txt");
    // const TEST_INPUT2: &str = include_str!("test_input2.txt");

    #[test]
    fn test_is_start_marker() {
        assert!(is_start_marker("abcd".as_bytes()));
        assert!(!is_start_marker("abad".as_bytes()));
        assert!(!is_start_marker("abca".as_bytes()));
        assert!(!is_start_marker("abcc".as_bytes()));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
        assert_eq!(part1(INPUT), 1578);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2178);
    }
}
