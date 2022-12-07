use cark_aoc_helper::*;
use day07::*;

fn main() {
    let fs = exec_printing_duration("Parsing", || parse(INPUT));
    exec_and_print("Part1", || part1(&fs));
    exec_and_print("Part2", || part2(&fs));
}
