use cark_aoc_helper::*;
use day16::*;

fn main() {
    let cave = exec_printing_duration("Parsing:", || Cave::parse(INPUT));

    exec_and_print("Part1", || part1(&cave));
    exec_and_print("Part2", || part2(&cave));
}
