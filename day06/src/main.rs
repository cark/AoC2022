const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("First method:");
    let (part1, duration) = with_timing(|| solve::<4>(INPUT));
    println!("Part1: {} in {} µs", part1, duration);
    let (part2, duration) = with_timing(|| solve::<14>(INPUT));
    println!("Part2: {} in {} µs", part2, duration);

    println!("\nSecond method:");
    let (part1, duration) = with_timing(|| solve_faster::<4>(INPUT));
    println!("Part1: {} in {} µs", part1, duration);
    let (part2, duration) = with_timing(|| solve_faster::<14>(INPUT));
    println!("Part2: {} in {} µs", part2, duration);
}

fn with_timing<Result: std::fmt::Display>(f: impl Fn() -> Result) -> (Result, u128) {
    let start_time = std::time::Instant::now();
    let result = f();
    let duration = start_time.elapsed().as_micros();
    (result, duration)
}

fn solve<const WINDOW_SIZE: usize>(input: &str) -> usize {
    let (window_index, _) = input
        .as_bytes()
        .windows(WINDOW_SIZE)
        .enumerate()
        .find(|(_, window)| is_start_marker(window))
        .unwrap();
    window_index + WINDOW_SIZE
}

fn is_start_marker(window: &[u8]) -> bool {
    (0..window.len())
        .map(|i| (window[i], &window[i + 1..]))
        .all(|(item, slice)| !slice.contains(&item))
}

fn solve_faster<const WINDOW_SIZE: usize>(input: &str) -> usize {
    let mut slider = Slider::<WINDOW_SIZE>::new();
    for &byte in input.as_bytes().iter() {
        if slider.add_byte(byte) {
            break;
        }
    }
    slider.index
}

struct Slider<const C: usize> {
    circular_window: [u8; C],
    byte_counts: [u8; 256],
    duplicate_count: usize,
    index: usize,
}

impl<const C: usize> Slider<C> {
    fn new() -> Self {
        Self {
            circular_window: [0; C],
            byte_counts: [0; 256],
            duplicate_count: 0,
            index: 0,
        }
    }

    fn add_byte(&mut self, byte: u8) -> bool {
        // remove
        let last_item = &mut self.circular_window[self.index % C];
        if self.index >= C {
            let byte_occurences = &mut self.byte_counts[*last_item as usize];
            if *byte_occurences > 1 {
                self.duplicate_count -= 1;
            }
            *byte_occurences -= 1;
        }
        // add
        *last_item = byte;
        let byte_occurences = &mut self.byte_counts[byte as usize];
        *byte_occurences += 1;
        if *byte_occurences > 1 {
            self.duplicate_count += 1;
        }
        self.index += 1;
        self.duplicate_count == 0 && self.index > C
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_start_marker() {
        assert!(is_start_marker("abcd".as_bytes()));
        assert!(!is_start_marker("abad".as_bytes()));
        assert!(!is_start_marker("abca".as_bytes()));
        assert!(!is_start_marker("abcc".as_bytes()));
    }

    #[test]
    fn test_part1() {
        let strings = [
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
            INPUT,
        ];
        let results = [7, 5, 6, 10, 11, 1578];
        strings
            .iter()
            .zip(results.iter())
            .for_each(|(&s, &r)| assert_eq!(solve::<4>(s), r));
        strings
            .iter()
            .zip(results.iter())
            .for_each(|(&s, &r)| assert_eq!(solve_faster::<4>(s), r));
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve::<14>(INPUT), 2178);
        assert_eq!(solve_faster::<14>(INPUT), 2178);
    }
}
