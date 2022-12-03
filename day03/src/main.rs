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

fn part1(input: &str) -> i32 {
    input.trim().lines().map(comp_common_sum).sum()
}

fn comp_common_sum(line: &str) -> i32 {
    let (comp1, comp2) = split_compartments(line.as_bytes());
    common_items(comp1, comp2)
        .iter()
        .copied()
        .map(char_to_priority)
        .sum()
}

fn part2(input: &str) -> i32 {
    let lines = input.trim().lines().map(str::as_bytes).collect::<Vec<_>>();
    lines.chunks(3).fold(0i32, |result, chunk| {
        let common = common_items(chunk[0], chunk[1]);
        let common2 = common_items(chunk[2], &common);
        result + common2.into_iter().map(char_to_priority).sum::<i32>()
    })
}

fn common_items(slice1: &[u8], slice2: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(slice1.len());
    slice1
        .iter()
        .filter_map(|&char| slice2.iter().find(|&c| *c == char))
        .for_each(|c| distinct_push(&mut result, *c));
    result
}

fn distinct_push(vec: &mut Vec<u8>, item: u8) {
    if !vec.contains(&item) {
        vec.push(item)
    }
}

fn char_to_priority(char: u8) -> i32 {
    if char >= b'a' && char <= b'z' {
        (char - b'a' + 1) as i32
    } else {
        (char - b'A' + 27) as i32
    }
}

fn split_compartments(sack: &[u8]) -> (&[u8], &[u8]) {
    (&sack[0..sack.len() / 2], &sack[sack.len() / 2..sack.len()])
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_split() {
        let rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp".as_bytes();
        assert_eq!(
            split_compartments(rucksack),
            ("vJrwpWtwJgWr".as_bytes(), "hcsFMMfFFhFp".as_bytes())
        );
    }

    #[test]
    fn test_common_items() {
        let rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp".as_bytes();
        let (comp1, comp2) = split_compartments(rucksack);
        assert_eq!(common_items(comp1, comp2), [b'p']);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT), 157);
        assert_eq!(part1(INPUT), 8139);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_INPUT), 70);
        assert_eq!(part2(INPUT), 2668);
    }
}
