use cark_aoc_helper::*;
use day06::*;

fn main() {
    println!("First method:");
    exec_and_print("Part1", || solve::<PACKET_MARKER_SIZE>(INPUT));
    exec_and_print("Part2", || solve::<MESSAGE_MARKER_SIZE>(INPUT));

    println!();
    println!("Second method:");
    exec_and_print("Part1", || solve_faster::<PACKET_MARKER_SIZE>(INPUT));
    exec_and_print("Part2", || solve_faster::<MESSAGE_MARKER_SIZE>(INPUT));
}
