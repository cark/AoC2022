use std::convert::identity;

pub const INPUT: &str = include_str!("input.txt");

type Pos = (i64, i64);
type Edge = (Pos, Pos);

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

pub fn better_part2(input: &str, max_pos: Pos) -> u64 {
    let sensors = parse(input);
    // find edges of all diamond shaped scanning areas
    let edges = edges(&sensors);
    let mut interesting_ys = Vec::with_capacity(edges.len() * edges.len() * 4);
    for i in 0..edges.len() {
        for j in 0..edges.len() {
            // find intersections of all segments with all other segments
            // keeping only the y values
            points_of_interest(&edges, i, j)
                .into_iter()
                .filter_map(identity)
                .for_each(|y| {
                    if y >= 0 && y <= max_pos.1 {
                        interesting_ys.push(y);
                    }
                });
        }
    }
    // iterate over our interesting lines, skipping duplicates
    let mut last_y = -1;
    for y in interesting_ys.into_iter() {
        if y == last_y {
            continue;
        }
        last_y = y;
        // now it's like brute force part2
        let mut ranges = sensor_ranges_at_line(&sensors, y);
        ranges.sort_unstable_by_key(|r| r.start);
        let merged = merge_ranges(&ranges);
        if merged.len() > 1 {
            let x = merged[0].end as u64 + 1;
            let y = y as u64;
            return 4000000 * x + y;
        }
    }
    unreachable!()
}

pub fn part2(input: &str, max_pos: Pos) -> u64 {
    // Naive version, same as part1 for each line,
    // only we're now looking for a line with 2 merged ranges.
    // the hole is between these 2 ranges
    let sensors = parse(input);
    for y in 0..=max_pos.1 {
        let mut ranges = sensor_ranges_at_line(&sensors, y);
        ranges.sort_unstable_by_key(|r| r.start);
        let merged = merge_ranges(&ranges);
        if merged.len() > 1 {
            let x = merged[0].end as u64 + 1;
            let y = y as u64;
            //println!("({x}, {y})"); // 2639657
            return 4000000 * x + y;
        }
    }
    unreachable!();
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

/// We return the edges of ou diamons shapes. First the two looking like this: /.
/// Then the two looking like this: \.
/// The edges always start with the smallest x value.
fn edges(sensors: &[Sensor]) -> Vec<Edge> {
    let mut edges = Vec::with_capacity(sensors.len() * 4);
    for sensor in sensors {
        let p = sensor.pos;
        let left = (p.0 - sensor.dist, p.1);
        let right = (p.0 + sensor.dist, p.1);
        let top = (p.0, p.1 - sensor.dist);
        let bottom = (p.0, p.1 + sensor.dist);
        edges.push((left, top));
        edges.push((bottom, right));
        edges.push((top, right));
        edges.push((left, bottom));
    }
    edges
}

/// We find the points of interest between 2 edges.
/// That is when they intersect.
fn points_of_interest(edges: &[Edge], index1: usize, index2: usize) -> [Option<i64>; 2] {
    if index1 == index2 {
        return [None; 2];
    }
    // we've been adding edges two by two having the same direction
    let edge1 = edges[index1];
    let edge2 = edges[index2];
    let params1 = line_params(edge1.0, edge1.1);
    let params2 = line_params(edge2.0, edge2.1);
    if params1.0 == params2.0 {
        // parallels
        return [None; 2];
    }
    let [y1, y2] = y_intersection(line_params(edge1.0, edge1.1), line_params(edge2.0, edge2.1));
    [
        y1.and_then(|y| (edge_contains_y(edge1, y) && edge_contains_y(edge2, y)).then_some(y)),
        y2.and_then(|y| (edge_contains_y(edge1, y) && edge_contains_y(edge2, y)).then_some(y)),
    ]
}

fn edge_contains_y(((_, y1), (_, y2)): Edge, y: i64) -> bool {
    y >= y1.min(y2) && y <= y1.max(y2)
}

fn line_params(p1: Pos, p2: Pos) -> (i64, i64) {
    //A line has this formula y = ax + b
    //in our case the slope is always 1 or -1 so ints are ok
    let a = (p2.1 - p1.1) / (p2.0 - p1.0);
    debug_assert!(a == 1 || a == -1);
    let b = p1.1 - a * p1.0;
    (a, b)
}

fn y_intersection((a1, b1): (i64, i64), (a2, b2): (i64, i64)) -> [Option<i64>; 2] {
    let top = a1 * b2 - a2 * b1;
    let bottom = a1 - a2;
    // discrete maths, don't we love it
    if top % bottom == 0 {
        [Some(top / bottom), None]
    } else {
        let result = top / bottom;
        // is this correct?
        [Some(result), Some(result + 1)]
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_parsing() {
        parse(TEST_INPUT);
    }

    #[test]
    fn test_line_params() {
        let a = line_params((2, 0), (4, 2));
        assert_eq!(a, (1, -2));
    }

    #[test]
    fn test_y_intersection() {
        let a = y_intersection((-1, 2), (1, -2));
        assert_eq!(a, [Some(0), None]);
        let a = y_intersection(line_params((1, 1), (4, 4)), line_params((1, 3), (3, 1)));
        assert_eq!(a, [Some(2), None]);
        let a = y_intersection(line_params((0, 0), (3, 3)), line_params((0, 3), (3, 0)));
        assert_eq!(a, [Some(1), Some(2)]);
    }

    #[test]
    fn test_points_of_interest() {
        let mut edges = vec![];
        edges.push(((0, 2), (2, 0)));
        edges.push(((2, 4), (4, 2)));
        edges.push(((0, 2), (2, 4)));
        edges.push(((2, 0), (4, 2)));
        assert_eq!(points_of_interest(&edges, 0, 0), [None, None]);
        assert_eq!(points_of_interest(&edges, 0, 1), [None, None]);
        assert_eq!(points_of_interest(&edges, 0, 2), [Some(2), None]);
        assert_eq!(points_of_interest(&edges, 0, 3), [Some(0), None]);
        let mut all_points = BTreeSet::new();
        for i in 0..edges.len() {
            for j in 0..edges.len() {
                points_of_interest(&edges, i, j)
                    .into_iter()
                    .filter_map(identity)
                    .for_each(|y| {
                        all_points.insert(y);
                    });
            }
        }
        println!("{:?}", all_points);
        assert_eq!(all_points.len(), 3);
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
            let mut ranges = sensor_ranges_at_line(&sensors, y);
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
        //assert_eq!(part2(INPUT, (4000000, 4000000)), 13743542639657);
    }
    #[test]
    fn test_better_part2() {
        assert_eq!(better_part2(TEST_INPUT, (20, 20)), 56000011);
        assert_eq!(better_part2(INPUT, (4000000, 4000000)), 13743542639657);
    }
}
