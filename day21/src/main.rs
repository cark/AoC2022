use cark_aoc_helper::*;
use day21::*;

fn main() {
    let mut troop = exec_printing_duration("Parsing", || Troop::parse(INPUT));
    exec_and_print("Part1", || part1(&troop));
    exec_and_print("Part2", move || part2(&mut troop));
}
