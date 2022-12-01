const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("part1: {}", find_biggest(totals(parse_input(INPUT))));
    println!("part2: {}", sum_3_biggest(totals(parse_input(INPUT))));
}

fn sum_3_biggest(mut elves: Vec<i32>) -> i32 {
    elves.sort();
    elves.iter().rev().take(3).sum::<i32>()
}

fn find_biggest(elves: Vec<i32>) -> i32 {
    elves.into_iter().max().unwrap()
}

fn totals(elves: Vec<Vec<i32>>) -> Vec<i32> {
    elves.iter().map(|elf| elf.iter().sum()).collect()
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut result = vec![];

    let mut lines = input.trim().lines();
    let mut current_vec = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            result.push(current_vec.clone());
            current_vec.clear()
        } else {
            current_vec.push(line.parse().unwrap())
        }
    }
    result.push(current_vec);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_prob1() {
        let biggest = find_biggest(totals(parse_input(SAMPLE_INPUT)));
        assert_eq!(biggest, 24000);
    }

    #[test]
    fn test_prob2() {
        let biggest = sum_3_biggest(totals(parse_input(SAMPLE_INPUT)));
        assert_eq!(biggest, 45000);
    }
}
