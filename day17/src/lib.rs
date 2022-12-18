pub const INPUT: &str = include_str!("input.txt");
const ROCKS: &str = include_str!("shapes.txt");
const FIELD_WIDTH: usize = 7;
const ACTUAL_WIDTH: usize = FIELD_WIDTH + 2;
const ROCK_TYPE_COUNT: usize = 5;

type Pos = (i32, i32);

fn add_pos(pos1: Pos, pos2: Pos) -> Pos {
    (pos1.0 + pos2.0, pos1.1 + pos2.1)
}

struct Rock {
    pixels: Vec<Pos>,
    size: Pos,
}

struct RockInstance<'a> {
    rock: &'a Rock,
    offset: Pos,
}

impl<'a> RockInstance<'a> {
    fn new(rock: &'a Rock, offset: Pos) -> Self {
        Self { rock, offset }
    }

    fn pixels(&self) -> impl Iterator<Item = Pos> + '_ {
        self.rock.pixels.iter().map(|&p| add_pos(p, self.offset))
    }

    fn offset_pixels(&self, offset: Pos) -> impl Iterator<Item = Pos> + '_ {
        self.pixels().map(move |p| {
            let result = add_pos(p, offset);
            //println!(" {:?}", result);
            result
        })
    }
}

struct Field {
    rows: Vec<[bool; ACTUAL_WIDTH]>,
    max_height: i32,
}

impl Field {
    fn new() -> Self {
        Self {
            rows: vec![],
            max_height: 1,
        }
    }

    fn occupied(&mut self, pos: Pos) -> bool {
        if pos.1 == 0 || pos.0 == 0 || pos.0 == ACTUAL_WIDTH as i32 - 1 {
            true
        } else {
            self.ensure_row(pos.1 as usize);
            self.rows[pos.1 as usize][pos.0 as usize]
        }
    }

    fn ensure_row(&mut self, y: usize) {
        if self.rows.len() < y {
            self.rows.resize(y + 1, [false; ACTUAL_WIDTH]);
        }
    }

    fn place_rock<'a, 'b>(&'a mut self, rock: &'b Rock) -> RockInstance<'b> {
        //println!("palce size: {:?}", rock.size);
        let offset = (3, self.max_height + 3);
        self.ensure_row((offset.1 + rock.size.1) as usize);
        RockInstance::new(rock, offset)
    }

    fn try_move(&mut self, ri: &mut RockInstance, offset: Pos) -> bool {
        if !ri.offset_pixels(offset).any(|p| self.occupied(p)) {
            //println!("occupied");
            ri.offset = add_pos(ri.offset, offset);
            true
        } else {
            false
        }
    }

    fn paint(&mut self, ri: RockInstance) {
        let h = ri.offset.1 + ri.rock.size.1;
        if h > self.max_height {
            self.max_height = h;
        }
        //println!("max_height: {}", self.max_height);
        ri.pixels()
            .for_each(|p| self.rows[p.1 as usize][p.0 as usize] = true);
    }

    fn print(&self, ri: &RockInstance) {
        for (y, line) in self.rows.iter().enumerate().rev() {
            println!();
            line.iter().enumerate().for_each(|(x, &b)| {
                let char = if b {
                    '#'
                } else {
                    if ri.pixels().find(|&p| p == (x as i32, y as i32)).is_some() {
                        '@'
                    } else {
                        '.'
                    }
                };
                print!("{}", char)
            });
        }
        println!();
    }

    fn print_line(&self, line: usize) {
        println!();
        self.rows[line].iter().for_each(|&b| {
            print!("{}", if b { '#' } else { ' ' });
        })
    }

    fn check_line(&self, ri: &RockInstance, iteration: usize) {
        for i in ri.offset.1..ri.offset.1 + ri.rock.size.1 {
            if self.rows[i as usize].iter().all(|&a| a) {
                println!("row[{i}] is full at iteration: {iteration}");
            }
        }
    }

    fn simulate<'a>(
        &mut self,
        mut rocks: impl Iterator<Item = &'a Rock>,
        mut jets: impl Iterator<Item = Pos>,
        iteration_count: usize,
    ) {
        let mut ri = self.place_rock(rocks.next().unwrap());
        let mut curr_iter = 0;
        while curr_iter < iteration_count {
            //println!("offset: {:?}", ri.offset);
            //self.print(&ri);
            self.try_move(&mut ri, jets.next().unwrap());
            if !self.try_move(&mut ri, (0, -1)) {
                self.paint(ri);
                ri = self.place_rock(rocks.next().unwrap());
                curr_iter += 1;
            }
            self.check_line(&ri, curr_iter);
            //self.print(&ri);
            //println!("{}", self.max_height);
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let mut field = Field::new();
    let rocks = parse_rocks(ROCKS);
    let rocks = rocks.iter().cycle();
    let (_, jets) = parse_jets(input);
    //field.simulate(rocks, jets, 2022);
    field.simulate(rocks, jets, 2022);
    field.max_height - 1
}

pub fn part2(input: &str) -> i32 {
    let mut field = Field::new();
    let rocks = parse_rocks(ROCKS);
    let rocks = rocks.iter().cycle();
    let (jet_count, jets) = parse_jets(input);
    //field.simulate(rocks, jets, 2022);
    // field.simulate(rocks, jets, jet_count * 5 * 100);
    // for i in 0..90 {
    //     field.print_line(i * jet_count * 5);
    // }
    field.max_height - 1
}

fn parse_rocks(input: &str) -> [Rock; 5] {
    let mut result = std::array::from_fn(|_| Rock {
        pixels: vec![],
        size: (0, 0),
    });
    let mut rock_index = 0;
    let mut lines = input.lines();
    let mut rock_line = 0;
    let mut pixels: Vec<(i32, i32)> = vec![];
    while rock_index < ROCK_TYPE_COUNT {
        let line = lines.next();
        if line.is_none() || line.unwrap().is_empty() {
            let (min_x, max_x) = pixels.iter().fold((i32::MAX, i32::MIN), |r, &pos| {
                (r.0.min(pos.0), r.1.max(pos.0))
            });
            let rock = &mut result[rock_index];
            rock.pixels = pixels.iter().map(|&p| (p.0, rock_line - p.1 - 1)).collect();
            rock.size = (1 + max_x - min_x, rock_line);
            rock_line = 0;
            pixels.clear();
            rock_index += 1;
        } else {
            let Some(line) = line else { panic!()};
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    pixels.push((x as i32, rock_line))
                }
            });
            rock_line += 1
        }
    }
    result
}

fn parse_jets<'a>(input: &'a str) -> (usize, impl Iterator<Item = Pos> + 'a) {
    let jets = input.as_bytes().iter().filter_map(|&c| match c {
        b'<' => Some((-1, 0)),
        b'>' => Some((1, 0)),
        _ => None,
    });
    (jets.clone().count(), jets.cycle())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    fn print_rocks() {
        let rocks = parse_rocks(ROCKS);
        for rock in &rocks {
            let mut pixels = [[' '; 5]; 5];
            rock.pixels
                .iter()
                .for_each(|pixel| pixels[pixel.1 as usize][pixel.0 as usize] = '#');
            for row in &pixels {
                println!();
                for char in row {
                    print!("{char}");
                }
            }
        }
        println!();
    }

    #[test]
    fn test_parse_shapes() {
        let rocks = parse_rocks(ROCKS);
        for (rock, pos) in rocks
            .into_iter()
            .zip([(4, 1), (3, 3), (3, 3), (1, 4), (2, 2)])
        {
            assert_eq!(rock.size, pos);
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3068);
        assert_eq!(part1(INPUT), 3219);
        // assert_eq!(part1(SHORT_INPUT), 10);
        // assert_eq!(part1(TEST_INPUT), 64);
        // assert_eq!(part1(INPUT), 4314);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 58);
        // assert_eq!(part2(INPUT), 2444);
    }
}
