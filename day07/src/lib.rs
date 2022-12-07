use std::collections::HashMap;

pub const INPUT: &str = include_str!("input.txt");
const MAX_USAGE: u64 = 70000000 - 30000000;

#[derive(Default, Debug, Clone, Copy)]
struct DirHandle(usize);

impl DirHandle {
    fn inc_mut(&mut self) {
        self.0 += 1
    }
}

#[derive(Default, Debug)]
struct Path<'a>(Vec<&'a str>);

impl<'a> Path<'a> {
    fn up(&mut self) {
        self.0.pop();
    }
    fn down<'b>(&'b mut self, fragment: &'a str) {
        self.0.push(fragment);
    }
    fn as_slice<'b>(&'b self) -> &'b [&'a str] {
        &self.0
    }
}

#[derive(Default, Debug)]
struct Directory {
    parent: Option<DirHandle>,
    total_size: u64,
}

#[derive(Default, Debug)]
pub struct FileSystem<'a> {
    directories: Vec<Directory>,
    path_to_handle: HashMap<Vec<&'a str>, DirHandle>,
    curr_handle: DirHandle,
}

impl<'a> FileSystem<'a> {
    fn dir_handle<'b>(&'b mut self, path: &'b [&'a str]) -> DirHandle {
        if let Some(&handle) = self.path_to_handle.get(path) {
            handle
        } else {
            let result = self.curr_handle;
            self.directories.push(Directory::default());
            self.path_to_handle.insert(Vec::from(path), result);
            self.curr_handle.inc_mut();
            result
        }
    }

    fn dir_mut(&mut self, handle: DirHandle) -> &mut Directory {
        &mut self.directories[handle.0]
    }

    fn add_size<'b>(&'b mut self, path: &'b [&'a str], size: u64) {
        let mut p = path;
        loop {
            let handle = self.dir_handle(p);
            self.dir_mut(handle).total_size += size;
            if p.len() == 0 {
                break;
            }
            p = &p[0..p.len() - 1];
        }
    }
}

#[derive(Default)]
struct Shell<'a> {
    fs: FileSystem<'a>,
    pwd: Path<'a>,
    dir_handle: Option<DirHandle>,
}

impl<'a> Shell<'a> {
    fn run_session<'b>(&'b mut self, input: &'a str) {
        for line in input.lines().filter(|line| !line.is_empty()) {
            let line = line.as_bytes();
            if line[0] == b'$' {
                self.process_command(&line[2..])
            } else {
                self.process_node(line)
            }
        }
    }

    fn process_command<'b>(&'b mut self, line: &'a [u8]) {
        if &line[0..2] == b"cd" {
            let rest = &line[3..];
            match rest {
                b"\\" => {
                    self.pwd = Path::default();
                    Self::update_dir_handle(self);
                }
                b".." => {
                    self.pwd.up();
                    Self::update_dir_handle(self);
                }
                bytes => {
                    self.pwd
                        .down(unsafe { std::str::from_utf8_unchecked(bytes) });
                    let handle = self.dir_handle;
                    self.update_dir_handle();
                    if let Some(child) = self.dir_handle {
                        self.fs.dir_mut(child).parent = handle;
                    }
                }
            }
        }
    }

    fn process_node<'b>(&'b mut self, line: &'a [u8]) {
        let mut parts = unsafe { std::str::from_utf8_unchecked(line) }.split_whitespace();
        let first = parts.next().unwrap();
        if first == "dir" {
            self.pwd.down(parts.next().unwrap());
            self.fs.dir_handle(&self.pwd.as_slice());
            self.pwd.up()
        } else {
            self.fs
                .add_size(self.pwd.as_slice(), first.parse::<u64>().unwrap());
        }
    }

    fn update_dir_handle<'b>(&'b mut self) {
        let handle = self.fs.dir_handle(&self.pwd.0);
        self.dir_handle = Some(handle);
    }
}

pub fn parse(input: &str) -> FileSystem {
    let mut shell = Shell::default();
    shell.run_session(input);
    shell.fs
}

pub fn part1(fs: &FileSystem) -> u64 {
    fs.directories
        .iter()
        .map(|dir| dir.total_size)
        .filter(|&size| size <= 100000)
        .sum()
}

pub fn part2(fs: &FileSystem) -> u64 {
    let total_used = fs.directories[0].total_size;
    let remove_at_least = total_used - MAX_USAGE;
    fs.directories
        .iter()
        .map(|dir| dir.total_size)
        .filter(|&size| size >= remove_at_least)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_shell() {
        let mut shell = Shell::default();
        shell.run_session(TEST_INPUT);
        assert_eq!(shell.fs.directories[0].total_size, 48381165);
    }

    #[test]
    fn test_part1() {
        let fs = parse(TEST_INPUT);
        assert_eq!(part1(&fs), 95437);
        let fs = parse(INPUT);
        assert_eq!(part1(&fs), 2104783);
    }

    #[test]
    fn test_part2() {
        let fs = parse(TEST_INPUT);
        assert_eq!(part2(&fs), 24933642);
        let fs = parse(INPUT);
        assert_eq!(part2(&fs), 5883165);
    }
}
