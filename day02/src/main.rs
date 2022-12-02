const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part1: {}", score(INPUT));
    println!("Part2: {}", part2(INPUT));
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn to_points(&self) -> i32 {
        *self as i32
    }

    fn round_outcome(&self, other: &Self) -> Outcome {
        use Outcome::*;
        use Shape::*;
        match (self, other) {
            (Rock, Paper) => Loss,
            (Paper, Scissors) => Loss,
            (Scissors, Rock) => Loss,
            (a, b) if a == b => Draw,
            (_, _) => Win,
        }
    }

    fn round_result(&self, other: &Self) -> i32 {
        self.to_points() + self.round_outcome(other).to_points()
    }

    fn from_char(c: char) -> Self {
        use Shape::*;
        match c {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissors,
            _ => unreachable!(),
        }
    }

    fn play_for_outcome(&self, outcome: &Outcome) -> Self {
        use Outcome::*;
        use Shape::*;
        match (self, outcome) {
            (s, Draw) => *s,
            (Rock, Win) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Win) => Rock,
            (Rock, Loss) => Scissors,
            (Paper, Loss) => Rock,
            (Scissors, Loss) => Paper,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl Outcome {
    fn from_char(c: char) -> Self {
        use Outcome::*;
        match c {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => unreachable!(),
        }
    }

    fn to_points(&self) -> i32 {
        *self as i32
    }
}

fn score_line(line: &str) -> i32 {
    let mut chars = line.chars();
    let other = Shape::from_char(chars.next().unwrap());
    chars.next();
    let me = Shape::from_char(chars.next().unwrap());
    me.round_result(&other)
}

fn score(input: &str) -> i32 {
    input.trim().lines().map(score_line).sum()
}

fn part2_score_line(line: &str) -> i32 {
    let mut chars = line.chars();
    let other = Shape::from_char(chars.next().unwrap());
    chars.next();
    let outcome = Outcome::from_char(chars.next().unwrap());
    let me = other.play_for_outcome(&outcome);
    let result = me.round_result(&other);
    result
}

fn part2(input: &str) -> i32 {
    input.trim().lines().map(part2_score_line).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_sample_input() {
        assert_eq!(score(SAMPLE_INPUT), 15);
    }

    #[test]
    fn test_part2_sample_input() {
        assert_eq!(part2(SAMPLE_INPUT), 12);
    }
}
