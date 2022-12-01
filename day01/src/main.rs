const INPUT: &str = include_str!("input.txt");

fn main() {
    let setup_time = std::time::Instant::now();
    let (part1, part2) = solve(INPUT);
    println!("solve time : {} µs", setup_time.elapsed().as_micros());
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    println!("total time: {} µs", setup_time.elapsed().as_micros());
}

fn solve(input: &str) -> (i32, i32) {
    let mut array = [0; 3];
    parse_input(input, |cals| {
        ordered_insert(&mut array, cals);
    });
    (array[0], array.into_iter().sum())
}

fn ordered_insert<const C: usize>(slice: &mut [i32; C], value: i32) {
    if slice[C - 1] < value {
        for i in 0..C {
            if slice[i] < value {
                slice.copy_within(i..(C - 1), i + 1);
                slice[i] = value;
                return;
            }
        }
    }
}

fn parse_input(input: &str, mut on_elf: impl FnMut(i32)) {
    let mut lines = input.trim().lines();
    let mut curr_total = 0;
    while let Some(line) = lines.next() {
        if line.is_empty() {
            on_elf(curr_total);
            curr_total = 0;
        } else {
            curr_total += line.parse::<i32>().unwrap();
        }
    }
    on_elf(curr_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_part1() {
        let result = solve(SAMPLE_INPUT).0;
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_part2() {
        let result = solve(SAMPLE_INPUT).1;
        assert_eq!(result, 45000);
    }
}
