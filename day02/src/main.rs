const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part1: {}", part1(INPUT));
    println!("Part2: {}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    input.trim().lines().map(part1_score_line).sum()
}

fn part1_score_line(line: &str) -> i32 {
    let mut chars = line.chars();
    let other = Shape::from_char(chars.next().unwrap());
    let me = Shape::from_char(chars.skip(1).next().unwrap());
    Round::new(me, other).to_points()
}

fn part2(input: &str) -> i32 {
    input.trim().lines().map(part2_score_line).sum()
}

fn part2_score_line(line: &str) -> i32 {
    let mut chars = line.chars();
    let other = Shape::from_char(chars.next().unwrap());
    let outcome = Outcome::from_char(chars.skip(1).next().unwrap());
    let me = other.play_for_outcome(outcome);
    Round::new(me, other).to_points()
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn to_points(self) -> i32 {
        self as i32
    }

    fn wins_against(self, other: Self) -> bool {
        other.to_winner() == self
    }

    fn to_winner(self) -> Self {
        use Shape::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn to_loser(self) -> Self {
        use Shape::*;
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn play_for_outcome(self, outcome: Outcome) -> Self {
        use Outcome::*;
        match outcome {
            Win => self.to_winner(),
            Loss => self.to_loser(),
            Draw => self,
        }
    }

    fn from_char(c: char) -> Self {
        use Shape::*;
        match c {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => unreachable!(),
        }
    }
}

struct Round {
    me: Shape,
    other: Shape,
}

impl Round {
    fn new(me: Shape, other: Shape) -> Self {
        Self { me, other }
    }

    fn outcome(&self) -> Outcome {
        if self.me.wins_against(self.other) {
            Outcome::Win
        } else if self.other.wins_against(self.me) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }

    fn to_points(&self) -> i32 {
        self.me.to_points() + self.outcome().to_points()
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

    fn to_points(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input.txt");

    #[test]
    fn test_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT), 15);
    }

    #[test]
    fn test_part2_sample_input() {
        assert_eq!(part2(SAMPLE_INPUT), 12);
    }
}
