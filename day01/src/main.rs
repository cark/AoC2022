const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    let mut result = 0;
    parse_input(input, |cals| {
        if cals > result {
            result = cals
        }
    });
    result
}

fn part2(input: &str) -> i32 {
    let mut result = [0; 3];
    parse_input(input, |cals| {
        ordered_insert(&mut result, cals);
    });
    result.into_iter().sum()
}

fn ordered_insert<const C: usize>(slice: &mut [i32; C], value: i32) {
    for i in 0..C {
        if slice[i] < value {
            slice.copy_within(i..(C - 1), i + 1);
            slice[i] = value;
            return;
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
        let result = part1(SAMPLE_INPUT);
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_part2() {
        let result = part2(SAMPLE_INPUT);
        assert_eq!(result, 45000);
    }
}
