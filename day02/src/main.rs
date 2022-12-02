const INPUT: &str = include_str!("input.txt");

fn main() {
    let setup_time = std::time::Instant::now();
    let part1 = part1(INPUT);
    let part1_dur = setup_time.elapsed().as_micros();
    let setup_time = std::time::Instant::now();
    let part2 = part2(INPUT);
    let part2_dur = setup_time.elapsed().as_micros();
    println!("Part1: {part1} in {part1_dur} µs");
    println!("Part2: {part2} in {part2_dur} µs");
}

fn part1(input: &str) -> i32 {
    solve(input, line_score::<Shape, Shape, Round>)
}

fn part2(input: &str) -> i32 {
    solve(input, line_score::<Shape, Outcome, Round>)
}

fn solve(input: &str, f: impl Fn(&str) -> i32) -> i32 {
    input.trim().lines().map(f).sum()
}

fn line_score<A, B, C>(line: &str) -> i32
where
    A: From<char>,
    B: From<char>,
    C: From<(A, B)> + Score,
{
    let mut chars = line.chars();
    let a = chars.next().unwrap().into();
    let b = chars.skip(1).next().unwrap().into();
    C::from((a, b)).score()
}

trait Score {
    fn score(&self) -> i32;
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
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
}

impl From<char> for Shape {
    fn from(value: char) -> Self {
        use Shape::*;
        match value {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => unreachable!(),
        }
    }
}

impl Score for Shape {
    fn score(&self) -> i32 {
        *self as i32
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl From<char> for Outcome {
    fn from(value: char) -> Self {
        use Outcome::*;
        match value {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => unreachable!(),
        }
    }
}

impl Score for Outcome {
    fn score(&self) -> i32 {
        *self as i32
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
}

impl Score for Round {
    fn score(&self) -> i32 {
        self.me.score() + self.outcome().score()
    }
}

impl From<(Shape, Shape)> for Round {
    fn from((other, me): (Shape, Shape)) -> Self {
        Self::new(me, other)
    }
}

impl From<(Shape, Outcome)> for Round {
    fn from((other, outcome): (Shape, Outcome)) -> Self {
        let me = other.play_for_outcome(outcome);
        Self::new(me, other)
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
