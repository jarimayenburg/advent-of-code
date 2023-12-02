use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Peekable;
use std::rc::{Rc, Weak};

use aoc_2022::read_lines;

#[derive(Debug)]
struct Node {
    name: String,
    size: u32,
    parent: Option<Weak<Node>>,
    children: Vec<Rc<Node>>,
}

impl Node {
    fn cd(&self, dir: &String) -> Rc<Node> {
        if dir == ".." {
            self.parent.unwrap().upgrade().unwrap()
        } else {
            Rc::clone(
                self.children
                    .iter()
                    .find(|child| child.name == *dir)
                    .expect("Child not found"),
            )
        }
    }

    fn ls<I: Iterator<Item = String>>(&mut self, lines: &mut Peekable<I>) {
        for line in lines {
            let (typesize, name) = line.split_once(" ").expect("Invalid line");

            if typesize == "$" {
                break;
            }

            let mut child = Node {
                name: name.to_string(),
                size: 0,
                children: vec![],
                parent: Some(Weak::new()),
            };

            if typesize == "dir" {
                child.size = str::parse(typesize).expect("Invalid file size");
            }

            self.add_child(child);
        }
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(Rc::new(child));
    }

    fn run_commands<I: Iterator<Item = String>>(&mut self, mut lines: Peekable<I>) {
        let words: Vec<String> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect();

        assert!(words[0] == "$");

        if words[1] == "ls" {
            self.ls(&mut lines);

            self.run_commands(lines);
        } else {
            let next = self.cd(&words[2]);

            next.run_commands(lines);
        }
    }
}

fn main() {
    let fs = read_filesystem();

    println!("{:?}", fs);
}

fn read_filesystem() -> Node {
    let lines = read_lines("inputs/day7.txt")
        .filter_map(|line| line.ok())
        .peekable();

    let mut root = Node {
        name: "/".to_owned(),
        size: 0,
        children: vec![],
        parent: None,
    };

    root.run_commands(lines);

    root
}
