pub const INPUT: &str = include_str!("input.txt");

use std::ops::RangeInclusive;

type Pos = (i32, i32);

fn manhattan(p1: Pos, p2: Pos) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    beacon: Pos,
    dist: i32,
}

fn parse_pos(s: &str) -> Pos {
    let numbers = s.split("x=").skip(1).next().unwrap();
    let mut numbers = numbers.split(',');
    let x = numbers.next().unwrap().parse().unwrap();
    let y = numbers
        .next()
        .unwrap()
        .split("y=")
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();
    (x, y)
}

fn parse(input: &str) -> Vec<Sensor> {
    let mut result = Vec::with_capacity(128);
    for line in input.lines().filter(|line| !line.is_empty()) {
        let mut split = line.split(':');
        let (sensor_str, beacon_str) = (split.next().unwrap(), split.next().unwrap());
        let sensor_pos = parse_pos(sensor_str);
        let beacon_pos = parse_pos(beacon_str);
        result.push(Sensor {
            beacon: beacon_pos,
            pos: sensor_pos,
            dist: manhattan(beacon_pos, sensor_pos),
        })
    }
    result
}

// fn cannot_contain_beacon(sensors: &[Sensor], pos: Pos) -> bool {
//     for sensor in sensors {
//         if sensor.beacon == pos {
//             return false;
//         } else {
//             if manhattan(pos, sensor.pos) <= sensor.dist {
//                 return true;
//             }
//         }
//     }
//     false
// }

// pub fn part1(input: &str, y: i32) -> u32 {
//     let sensors = parse(input);
//     let (min_x, max_x) = sensors
//         .iter()
//         .fold((i32::MAX, i32::MIN), |(min, max), sensor| {
//             (
//                 min.min(sensor.pos.0 - sensor.dist),
//                 max.max(sensor.pos.0 + sensor.dist),
//             )
//         });
//     (min_x..=max_x)
//         .filter(|x| cannot_contain_beacon(&sensors, (*x, y)))
//         .count() as u32
// }

fn ranges_at_line(sensors: &[Sensor], y: i32) -> Vec<RangeInclusive<i32>> {
    sensors
        .iter()
        .filter_map(move |sensor| {
            let half_size = sensor.dist - (y - sensor.pos.1).abs();
            let range_size = half_size * 2 + 1;
            if range_size > 0 {
                Some(sensor.pos.0 - half_size..=sensor.pos.0 + half_size)
            } else {
                None
            }
        })
        .collect()
}

fn merge_ranges(sorted_ranges: &[RangeInclusive<i32>]) -> Vec<RangeInclusive<i32>> {
    let mut result = Vec::with_capacity(sorted_ranges.len());
    let mut index = 0;
    let mut current = sorted_ranges.get(index).cloned();
    loop {
        let next = sorted_ranges.get(index);
        index += 1;
        match (current, next.cloned()) {
            (Some(r1), None) => {
                result.push(r1);
                return result;
            }
            (Some(r1), Some(r2)) if r1.contains(r2.start()) => {
                current = Some(*r1.start()..=*r1.end().max(&r2.end()))
            }
            (Some(r1), Some(r2)) => {
                current = Some(r2);
                result.push(r1);
            }
            (None, _) => return result,
        }
    }
}

// fn punch_range(
//     range: RangeInclusive<i32>,
//     sorted_holes: &[RangeInclusive<i32>],
//     into_vec: &mut Vec<RangeInclusive<i32>>,
// ) {
//     into_vec.clear();
//     let mut current = range;
//     for hole in sorted_holes {
//         match (hole.contains(current.start()), hole.contains(current.end())) {
//             (true, true) => return,
//             (true, false) => current = *hole.end() + 1..=*current.end(),
//             (false, true) => {
//                 into_vec.push(*current.start()..=*hole.start() - 1);
//                 return;
//             }
//             (false, false) => {
//                 into_vec.push(*current.start()..=*hole.start() - 1);
//                 current = *hole.end() + 1..=*current.end();
//             }
//         }
//     }
// }

fn beacons_in_range(
    sensors: &[Sensor],
    range: &RangeInclusive<i32>,
    y: i32,
    into_vec: &mut Vec<(i32, i32)>,
) {
    into_vec.clear();
    for s in sensors {
        if s.beacon.1 == y && range.contains(&s.beacon.0) && !into_vec.contains(&s.beacon) {
            into_vec.push(s.beacon);
        }
    }
}

pub fn part1(input: &str, y: i32) -> u32 {
    let sensors = parse(input);
    let mut ranges = ranges_at_line(&sensors, y);
    ranges.sort_unstable_by_key(|r| *r.start());
    let merged = merge_ranges(&ranges);
    let mut included_beacons = Vec::with_capacity(sensors.len());
    merged
        .iter()
        .map(|range| {
            beacons_in_range(&sensors, range, y, &mut included_beacons);
            (range.end() - range.start() + 1) as usize - included_beacons.len()
        })
        .sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_parsing() {
        parse(TEST_INPUT);
    }

    // #[test]
    // fn test_ranges_at_line() {
    //     let sensors = parse(TEST_INPUT);
    //     let ranges = ranges_at_line(&sensors, 10).collect::<Vec<_>>();
    //     println!("{ranges:?}");
    //     assert!(false);
    // }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT, 10), 26);
        assert_eq!(part1(INPUT, 2000000), 4748135);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(TEST_INPUT), 93);
    //     assert_eq!(part2(INPUT), 27426);
    // }
}
