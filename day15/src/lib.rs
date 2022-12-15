pub const INPUT: &str = include_str!("input.txt");

use std::{convert::identity, ops::RangeInclusive};

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

fn ranges_at_line<'a>(
    sensors: &'a [Sensor],
    y: i32,
) -> impl Iterator<Item = RangeInclusive<i32>> + 'a {
    sensors.iter().filter_map(move |sensor| {
        let half_size = sensor.dist - (y - sensor.pos.1).abs();
        let range_size = half_size * 2 + 1;
        if range_size > 0 {
            Some(sensor.pos.0 - half_size..=sensor.pos.0 + half_size)
        } else {
            None
        }
    })
}

fn merge_ranges(ranges: &mut [RangeInclusive<i32>], into_vec: &mut Vec<RangeInclusive<i32>>) {
    ranges.sort_by_key(|range| *range.start());
    into_vec.clear();
    let mut index = 0;
    let mut current = ranges.get(index).cloned();
    loop {
        let next = ranges.get(index);
        index += 1;
        match (current, next.cloned()) {
            (Some(r1), None) => {
                into_vec.push(r1);
                return;
            }
            (Some(r1), Some(r2)) if r1.contains(r2.start()) => {
                current = Some(*r1.start()..=*r1.end().max(&r2.end()))
            }
            (Some(r1), Some(r2)) => {
                current = Some(r2);
                into_vec.push(r1);
            }
            (None, _) => return,
        }
    }
}

fn punch_range(
    range: RangeInclusive<i32>,
    sorted_holes: &[RangeInclusive<i32>],
    into_vec: &mut Vec<RangeInclusive<i32>>,
) {
    into_vec.clear();
    let mut current = range;
    for hole in sorted_holes {
        match (hole.contains(current.start()), hole.contains(current.end())) {
            (true, true) => return,
            (true, false) => current = *hole.end() + 1..=*current.end(),
            (false, true) => {
                into_vec.push(*current.start()..=*hole.start() - 1);
                return;
            }
            (false, false) => {
                into_vec.push(*current.start()..=*hole.start() - 1);
                current = *hole.end() + 1..=*current.end();
            }
        }
    }
}

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
    let mut ranges = Vec::with_capacity(sensors.len());
    ranges_at_line(&sensors, y).for_each(|r| ranges.push(r));
    //println!("ranges: len={} data={:?}", ranges.len(), ranges);
    let mut merged = Vec::with_capacity(sensors.len());
    merge_ranges(&mut ranges, &mut merged);
    //println!("merged ranges: len={} data={:?}", merged.len(), merged);
    // let (min_x, max_x) = sensors
    //     .iter()
    //     .fold((i32::MAX, i32::MIN), |(min, max), sensor| {
    //         (
    //             min.min(sensor.pos.0 - sensor.dist),
    //             max.max(sensor.pos.0 + sensor.dist),
    //         )
    //     });

    let mut included_beacons = Vec::with_capacity(sensors.len());
    merged
        .iter()
        .map(|range| {
            //println!("{}", (range.end() - range.start() + 1));
            beacons_in_range(&sensors, range, y, &mut included_beacons);
            (range.end() - range.start() + 1) as usize - included_beacons.len()
        })
        .sum::<usize>() as u32

    // sensors.iter().fold(merged.len(), |result, sensor| {
    //     if sensor.beacon.1 == y && {
    //         result - 1
    //     } else {
    //         result
    //     }
    // })
    // let mut punched = Vec::with_capacity(sensors.len());
    // punch_range(min_x..=max_x, &merged, &mut punched);
    // println!("punched: len={} data={:?}", punched.len(), punched);
    // todo!()
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
