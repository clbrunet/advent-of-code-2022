use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug, Default)]
struct Directory {
    parent: Weak<Self>,
    files: RefCell<Vec<u32>>,
    directories: RefCell<HashMap<String, Rc<Self>>>,
    size: RefCell<Option<u32>>,
}

impl Directory {
    fn new(parent: Weak<Self>) -> Rc<Self> {
        Rc::new(Self {
            parent,
            files: RefCell::default(),
            directories: RefCell::default(),
            size: RefCell::new(None),
        })
    }

    fn cd(self: &Rc<Self>, name: &str) -> Rc<Self> {
        match name {
            "/" => {
                let mut root = Rc::clone(self);
                while let Some(parent) = self.parent.upgrade() {
                    root = parent;
                }
                root
            }
            ".." => self.parent.upgrade().unwrap(),

            child => Rc::clone(self.directories.borrow().get(child).unwrap()),
        }
    }

    fn mkdir(self: &Rc<Self>, name: impl Into<String>) {
        self.directories
            .borrow_mut()
            .insert(name.into(), Self::new(Rc::downgrade(self)));
    }

    fn touch(self: &Rc<Self>, size: u32) {
        self.files.borrow_mut().push(size);
    }

    fn set_sizes(self: &Rc<Self>) -> u32 {
        let mut size = self.size.borrow_mut();
        *size = Some(self.files.borrow().iter().sum::<u32>()
        + self.directories.borrow().iter().map(|(_, a)| {
                a.set_sizes()
            }).sum::<u32>());
        size.unwrap()
    }

    fn get_part_1_result(self: &Rc<Self>) -> u32 {
        let result = self.directories.borrow().iter().map(|(_, directory)| {
                directory.get_part_1_result()
            }).sum::<u32>();
        let size = self.size.borrow().unwrap();
        if size < 100000 {
            result + size
        } else {
            result
        }
    }

    fn set_part_2_result(self: &Rc<Self>, min: u32, result: &mut u32) {
        for (_, directory) in self.directories.borrow().iter() {
            directory.set_part_2_result(min, result);
        }
        let size = self.size.borrow().unwrap();
        if size >= min && size < *result {
            *result = size;
        }
    }
}

fn create_tree(input: &str) -> Rc<Directory> {
    let root = Directory::new(Weak::new());
    let mut cwd = Rc::clone(&root);
    for line in input.lines() {
        let mut words = line.split_whitespace();
        match words.next().unwrap() {
            "$" => {
                if let Some("cd") = words.next() {
                    cwd = cwd.cd(words.next().unwrap());
                }
            }
            "dir" => cwd.mkdir(words.next().unwrap()),
            size => cwd.touch(size.parse().unwrap()),
        }
    }
    root.set_sizes();
    root
}

pub fn part_1(input: &str) -> String {
    let root = create_tree(input);
    root.get_part_1_result().to_string()
}

pub fn part_2(input: &str) -> String {
    let root = create_tree(input);
    let min = 30000000 - (70000000 - root.size.borrow().unwrap());
    let mut result = 70000000;
    root.set_part_2_result(min, &mut result);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "95437");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "24933642");
    }
}
