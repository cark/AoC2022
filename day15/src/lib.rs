pub const INPUT: &str = include_str!("input.txt");

type Pos = (i64, i64);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
    fn contains(&self, value: i64) -> bool {
        value >= self.start && value <= self.end
    }
}

fn manhattan(p1: Pos, p2: Pos) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    beacon: Pos,
    dist: i64,
}

pub fn part1(input: &str, y: i64) -> u64 {
    let sensors = parse(input);
    // get ranges covered by each sensor on this line
    let mut ranges = sensor_ranges_at_line(&sensors, y);

    // merge the ranges
    ranges.sort_unstable_by_key(|r| r.start);
    let merged = merge_ranges(&ranges);

    // Sum the sizes of all these ranges, taking care of removing
    // each beacon known to be on this line
    let mut included_beacons = Vec::with_capacity(sensors.len());
    merged
        .iter()
        .map(|range| {
            // there can be multiple beacons on a line
            beacons_in_range(&sensors, range, y, &mut included_beacons);
            (range.end - range.start + 1) as usize - included_beacons.len()
        })
        .sum::<usize>() as u64
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

fn sensor_ranges_at_line(sensors: &[Sensor], y: i64) -> Vec<Range> {
    sensors
        .iter()
        .filter_map(move |sensor| sensor_range_at_line(sensor, y))
        .collect()
}

fn sensor_range_at_line(sensor: &Sensor, y: i64) -> Option<Range> {
    let half_size = sensor.dist - (y - sensor.pos.1).abs();
    let range_size = half_size * 2 + 1;
    if range_size > 0 {
        Some(Range::new(
            sensor.pos.0 - half_size,
            sensor.pos.0 + half_size,
        ))
    } else {
        None
    }
}

fn merge_ranges(sorted_ranges: &[Range]) -> Vec<Range> {
    let mut result = Vec::with_capacity(sorted_ranges.len());
    let mut index = 0;
    let mut current = sorted_ranges.get(index).copied();
    loop {
        let next = sorted_ranges.get(index);
        index += 1;
        match (current, next) {
            (Some(r1), None) => {
                result.push(r1);
                return result;
            }
            (Some(r1), Some(r2)) if r1.contains(r2.start) => {
                current = Some(Range::new(r1.start, r1.end.max(r2.end)))
            }
            (Some(r1), Some(&r2)) => {
                if r1.end + 1 == r2.start {
                    current = Some(Range::new(r1.start, r1.end.max(r2.end)))
                } else {
                    current = Some(r2);
                    result.push(r1);
                }
            }
            (None, _) => return result,
        }
    }
}

fn beacons_in_range(sensors: &[Sensor], range: &Range, y: i64, into_vec: &mut Vec<(i64, i64)>) {
    into_vec.clear();
    for s in sensors {
        if s.beacon.1 == y && range.contains(s.beacon.0) && !into_vec.contains(&s.beacon) {
            into_vec.push(s.beacon);
        }
    }
}

pub fn part2(input: &str, max_pos: Pos) -> u64 {
    let sensors = parse(input);
    for y in 0..=max_pos.1 {
        let mut ranges = sensor_ranges_at_line(&sensors, y)
            .iter()
            .filter_map(|r| {
                if r.end < 0 {
                    None
                } else if r.start > max_pos.0 {
                    None
                } else {
                    Some(Range::new(r.start.max(0), r.end.min(max_pos.0)))
                }
            })
            .collect::<Vec<_>>();
        ranges.sort_unstable_by_key(|r| r.start);
        let merged = merge_ranges(&ranges);
        if merged.len() > 1 {
            let x = merged[0].end as u64 + 1;
            let y = y as u64;
            return 4000000 * x + y;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_parsing() {
        parse(TEST_INPUT);
    }

    #[test]
    fn test_sensor_range_at_line() {
        let sensors = parse(TEST_INPUT);
        let sensor = &sensors[6];
        let results = [
            1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 17, 15, 13, 11, 9, 7, 5, 3, 1,
        ];
        for (y, result) in (-2..16).zip(results) {
            let r = sensor_range_at_line(&sensor, y).unwrap();
            assert_eq!(r.end - r.start + 1, result);
        }
        println!("{:?}", sensors[6]);
    }

    #[test]
    fn draw_covered() {
        let sensors = parse(TEST_INPUT);
        println!("using sensor_ranges_at_line");
        for y in 0..20 {
            let mut chars = [b' '; 21];
            sensor_ranges_at_line(&sensors, y).iter().for_each(|r| {
                (r.start..=r.end).for_each(|x| {
                    if x >= 0 && x <= 20 {
                        chars[x as usize] = b'#';
                    }
                });
            });
            println!("{}", std::str::from_utf8(&chars).unwrap());
        }
        println!();
        println!("using merged ranges");
        for y in 0..20 {
            let mut ranges = sensor_ranges_at_line(&sensors, y)
                .iter()
                .filter_map(|r| {
                    if r.end < 0 {
                        None
                    } else if r.start > 20 {
                        None
                    } else {
                        Some(Range::new(r.start.max(0), r.end.min(20)))
                    }
                })
                .collect::<Vec<_>>();
            ranges.sort_unstable_by_key(|r| r.start);
            let mut chars = [b' '; 21];
            let merged = merge_ranges(&ranges);
            print!("{}", merged.len());
            if merged.len() > 1 {
                print!("{:?}", &merged);
            }
            merged.iter().for_each(|r| {
                (r.start..=r.end).for_each(|x| {
                    if x >= 0 && x <= 20 {
                        chars[x as usize] = b'#';
                    }
                });
            });
            println!("{}", std::str::from_utf8(&chars).unwrap());
        }
        //assert!(false)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT, 10), 26);
        assert_eq!(part1(INPUT, 2000000), 4748135);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT, (20, 20)), 56000011);
        assert_eq!(part2(INPUT, (4000000, 4000000)), 13743542639657);
    }
}
