use cark_aoc_helper::*;
use day15::*;

fn main() {
    exec_and_print("Part1", || part1(INPUT, 2000000));
    exec_and_print("Part2", || part2(INPUT, (4000000, 4000000)));
    exec_and_print("Better Part2", || better_part2(INPUT, (4000000, 4000000)));
}
