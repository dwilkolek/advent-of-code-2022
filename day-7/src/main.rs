// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
use std::cell::RefCell;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;

trait Sizeable {
    fn size(&self) -> usize;
}

impl Sizeable for Directory {
    fn size(&self) -> usize {
        let mut total = 0;
        for child in self.children.clone() {
            total = total + child.as_ref().borrow().size()
        }
        for file in self.files.iter() {
            total = total + file.size
        }
        return total;
    }
}

impl Sizeable for File {
    fn size(&self) -> usize {
        return self.size;
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    children: Vec<Rc<RefCell<Directory>>>,
    files: Vec<File>,
}

impl Directory {
    fn root() -> Directory {
        Directory {
            name: String::from("/"),
            parent: None,
            children: Vec::new(),
            files: Vec::new(),
        }
    }
    fn add_child(&mut self, child: Rc<RefCell<Directory>>) {
        self.children.push(child);
    }

    fn new(name: String, parent: Rc<RefCell<Directory>>) -> Directory {
        println!("Creating dir {}, {} ", name, parent.as_ref().borrow().name);
        Directory {
            name,
            parent: Some(parent),
            children: Vec::new(),
            files: Vec::new(),
        }
    }
    fn add_file(&mut self, file: File) {
        self.files.push(file)
    }
    fn get_children(&self, name: String) -> Option<Rc<RefCell<Directory>>> {
        self.children
            .clone()
            .into_iter()
            .find(|child| child.as_ref().borrow().name == name)
    }

    fn get_file(&self, name: String) -> Option<bool> {
        match self
            .files
            .clone()
            .into_iter()
            .find(|child| child.name == name)
        {
            Some(_) => Some(true),
            None => None,
        }
    }
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: String, size: usize) -> File {
        File { name, size }
    }
}

fn main() {
    let mut directories: Vec<Rc<RefCell<Directory>>> = vec![];
    let root = Rc::new(RefCell::new(Directory::root()));
    let mut current = Rc::clone(&root);

    let mut path: Vec<String> = vec![];
    if let Ok(lines) = read_lines("input.txt") {
        let cmd_dump: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();

        for cmd in cmd_dump.into_iter() {
            let cmd_parts: Vec<&str> = cmd.split(" ").into_iter().collect();
            if cmd.starts_with("$") {
                //command

                println!("{} # {}", path.join(" / "), cmd);
                match cmd_parts[1] {
                    "cd" => match cmd_parts[2] {
                        ".." => {
                            let current_clone = Rc::clone(&current);
                            let parent = Rc::clone(
                                current_clone
                                    .as_ref()
                                    .borrow()
                                    .parent
                                    .as_ref()
                                    .expect("Parent is missing"),
                            );
                            path.pop();
                            current = parent;
                        }
                        "/" => {
                            current = {
                                path = vec![];
                                Rc::clone(&root)
                            }
                        }
                        name => {
                            path.push(name.to_owned());
                            let already_exists = current.borrow_mut().get_children(name.to_owned());
                            match already_exists {
                                Some(child) => current = Rc::clone(&child),
                                None => {
                                    let child = Rc::new(RefCell::new(Directory::new(
                                        String::from(name),
                                        Rc::clone(&current),
                                    )));
                                    current.borrow_mut().add_child(Rc::clone(&child));
                                    directories.push(Rc::clone(&child));
                                    current = Rc::clone(&child)
                                }
                            }
                        }
                    },
                    "ls" => {
                        println!("NOOP LS")
                    }
                    _ => (),
                }
            } else {
                println!("  --- {}", cmd);
                if cmd_parts[0] == "dir" {
                    let name = cmd_parts[1];
                    let already_exists = current.borrow_mut().get_children(name.to_owned());
                    match already_exists {
                        None => {
                            let child = Rc::new(RefCell::new(Directory::new(
                                String::from(name),
                                Rc::clone(&current),
                            )));
                            directories.push(Rc::clone(&child));
                            current.borrow_mut().add_child(Rc::clone(&child));
                        }
                        _ => {
                            println!("NOOP")
                        }
                    }
                } else {
                    let size: usize = cmd_parts[0].to_string().parse().unwrap();
                    let name = String::from(cmd_parts[1]);
                    let has_file = current.as_ref().borrow().get_file(name.clone());
                    match has_file {
                        None => {
                            current
                                .as_ref()
                                .borrow_mut()
                                .add_file(File::new(name, size));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    let below_100000: usize = directories
        .clone()
        .into_iter()
        .filter(|dir| dir.as_ref().borrow().size() < 100000)
        .map(|dir| dir.as_ref().borrow().size())
        .sum();

    println!("Total of directories below 100000: {}", below_100000);

    //Part 2
    let total_disk_space = 70000000;
    let free_required = 30000000;
    let unused = total_disk_space - root.as_ref().borrow().size();
    let to_delete = free_required - unused;
    let mut matching_dirs_sizes: Vec<usize> = directories
        .clone()
        .into_iter()
        .filter(|dir| dir.as_ref().borrow().size() >= to_delete)
        .map(|dir| dir.as_ref().borrow().size())
        .collect();
    matching_dirs_sizes.sort();
    matching_dirs_sizes.first().unwrap();

    println!(
        "Should delete directory of size: {}",
        matching_dirs_sizes.first().unwrap()
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
