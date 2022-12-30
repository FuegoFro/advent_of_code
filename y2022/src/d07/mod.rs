
use std::collections::{HashMap};

use std::string::ToString;

#[derive(Debug)]
enum Entry {
    File { size: usize },
    Directory { directory: Directory },
}

impl Entry {
    fn file(size: usize) -> Self {
        Self::File { size }
    }

    fn directory() -> Self {
        Self::Directory {
            directory: Directory::new(),
        }
    }
}

#[derive(Debug)]
struct Directory {
    size: usize,
    children: HashMap<String, Box<Entry>>,
}

impl Directory {
    fn new() -> Self {
        Directory {
            size: 0,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, path: &[&str], name: impl ToString, entry: Entry) {
        let added_size = match entry {
            Entry::File { size } => size,
            Entry::Directory { ref directory } => directory.size,
        };
        self.size += added_size;

        if let Some((next, rest)) = path.split_first() {
            let child = self
                .children
                .entry(next.to_string())
                .or_insert_with(|| Box::new(Entry::directory()));
            match &mut (**child) {
                Entry::Directory { ref mut directory } => directory.insert(rest, name, entry),
                _ => panic!("Expected dir"),
            };
        } else {
            self.children
                .entry(name.to_string())
                .or_insert_with(|| Box::new(entry));
        };
    }

    fn dir_sizes<'a>(&'a self) -> Box<dyn Iterator<Item = usize> + 'a> {
        Box::new(
            std::iter::once(self.size).chain(
                self.children
                    .values()
                    .filter_map(|e| match &**e {
                        Entry::Directory { directory } => Some(directory),
                        _ => None,
                    })
                    .flat_map(|d| d.dir_sizes()),
            ),
        )
    }
}

const CD_COMMAND_PREFIX: &str = "$ cd ";
const LS_COMMAND: &str = "$ ls";

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let mut root = Directory::new();

    let mut pwd = vec![];
    for line in input.split('\n') {
        if let Some(dir) = line.strip_prefix(CD_COMMAND_PREFIX) {
            if dir == "/" {
                pwd.clear();
            } else if dir == ".." {
                pwd.pop();
            } else {
                pwd.push(dir);
            }
        } else if line != LS_COMMAND {
            // Kinda just assuming we're either cd'ing, ls'ing, or seeing the output of ls.
            let (metadata, name) = line.split_once(' ').unwrap();
            let entry = if metadata == "dir" {
                Entry::directory()
            } else {
                Entry::file(metadata.parse().unwrap())
            };
            root.insert(&pwd, name, entry);
        }
    }

    let pt1_total: usize = root.dir_sizes().filter(|s| s < &100000).sum();

    println!("Part 1: {}", pt1_total);

    let free_space = 70000000 - root.size;
    let needed_space = 30000000 - free_space;

    let smallest_sufficient_dir_size = root
        .dir_sizes()
        .filter(|s| s > &needed_space)
        .min()
        .unwrap();

    println!("Part 2: {}", smallest_sufficient_dir_size);
}
