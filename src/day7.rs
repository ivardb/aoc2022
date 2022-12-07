use std::collections::HashMap;
use std::fs;
use itertools::Itertools;
use crate::day7::Files::{Dir, File};

#[derive(Debug)]
enum Files {
    File(usize, String),
    Dir(String),
}

enum FileNode {
    File(usize, String),
    Dir(Directory)
}

impl FileNode {
    fn get_size(&mut self) -> usize {
        match self {
            FileNode::File(s, _) => {*s}
            FileNode::Dir(d) => {d.get_size()}
        }
    }

    fn test<F, G>(&self, f: &F, acc: G) -> G
    where F : Fn(&Directory, G) -> G {
        match self {
            FileNode::File(_, _) => acc,
            FileNode::Dir(d) => {
                let mut res = f(d, acc);
                for c in &d.children {
                    res = c.test(f, res);
                }
                res
            }
        }
    }
}

struct Directory {
    name: String,
    children: Vec<FileNode>,
    size: Option<usize>,
}

impl Directory {
    fn get_size(&mut self) -> usize {
        if let Some(s) = self.size {
            s
        } else {
            let mut size = 0;
            for f in &mut self.children {
                match f {
                    FileNode::File(s, _) => {size += *s}
                    FileNode::Dir(d) => {size += d.get_size()}
                }
            }
            self.size = Some(size);
            size
        }
    }
}

fn visualize(filesystem: &HashMap<String, Vec<Files>>, indentation: usize, dir: String) {
    println!("{n:|>width$}{dir}", n="", width=indentation);
    for file in filesystem.get(&dir).unwrap() {
        if let Dir(n) = file {
            visualize(filesystem, indentation + 1, dir.clone() + "/" + n);
        }
    }
}

fn construct_node(filesystem: &HashMap<String, Vec<Files>>, name: String) -> FileNode {
    FileNode::Dir(Directory {
        name: name.clone(),
        children: filesystem.get(&name).unwrap().iter().map(|f| match f {
            File(s, n) => {FileNode::File(*s, n.clone())}
            Dir(n) => {construct_node(filesystem, name.clone() + "/" + n)}
        }).collect_vec(),
        size: None})
}

pub fn day7() {
    let data = fs::read_to_string("data/day7.txt").unwrap();
    let mut filesystem = HashMap::new();
    let mut current_dir_list = Vec::new();
    let mut current_dir = String::from("");
    for line in data.split('\n') {
        let words = line.split_ascii_whitespace().collect_vec();
        match words[0] {
            "$" => {
                if words[1] == "cd" {
                    if words[2] == ".." {
                        current_dir_list.pop();
                    } else {
                        current_dir_list.push(String::from(words[2]));
                    }
                    current_dir = current_dir_list.join("/");
                    if !filesystem.contains_key(&current_dir) {
                        filesystem.insert(current_dir.clone(), Vec::new());
                    }
                }
            },
            "dir" => filesystem.get_mut(&current_dir).unwrap().push(Dir(String::from(words[1]))),
            a => filesystem.get_mut(&current_dir).unwrap().push(File(a.parse().unwrap(), String::from(words[1]))),
        }
    }
    let mut root = construct_node(&filesystem, String::from("/"));
    let remaining = 30000000 - (70000000 - root.get_size());

    println!("Part 1: {}", root.test(&|d, acc| acc + if d.size.unwrap() <= 100000 { d.size.unwrap()} else {0},0));
    println!("Part 2: {}", root.test(&|d, (m, r)| {
        let s = d.size.unwrap();
        if s >= remaining && s - remaining < m {
            (s - remaining, s)
        } else {
            (m, r)
        }
    }, (70000000, 0)).1);
}