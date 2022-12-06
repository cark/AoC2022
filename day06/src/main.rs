use day06::*;

fn main() {
    println!("First method:");
    let (part1, duration) = with_timing(|| solve::<PACKET_MARKER_SIZE>(INPUT));
    println!("Part1: {} in {} µs", part1, duration);
    let (part2, duration) = with_timing(|| solve::<MESSAGE_MARKER_SIZE>(INPUT));
    println!("Part2: {} in {} µs", part2, duration);

    println!("\nSecond method:");
    let (part1, duration) = with_timing(|| solve_faster::<PACKET_MARKER_SIZE>(INPUT));
    println!("Part1: {} in {} µs", part1, duration);
    let (part2, duration) = with_timing(|| solve_faster::<MESSAGE_MARKER_SIZE>(INPUT));
    println!("Part2: {} in {} µs", part2, duration);
}

fn with_timing<Solution: std::fmt::Display>(f: impl Fn() -> Solution) -> (Solution, u128) {
    let start_time = std::time::Instant::now();
    let result = f();
    let duration = start_time.elapsed().as_micros();
    (result, duration)
}
