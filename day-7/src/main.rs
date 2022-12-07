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
    parent: Option<Rc<RefCell<Directory>>>,
    children: Vec<Rc<RefCell<Directory>>>,
    files: Vec<File>,
}

impl Directory {
    fn root() -> Directory {
        Directory {
            parent: None,
            children: Vec::new(),
            files: Vec::new(),
        }
    }
    fn add_child(&mut self, child: Rc<RefCell<Directory>>) {
        self.children.push(child);
    }

    fn new(parent: Rc<RefCell<Directory>>) -> Directory {
        Directory {
            parent: Some(parent),
            children: Vec::new(),
            files: Vec::new(),
        }
    }
    fn add_file(&mut self, file: File) {
        self.files.push(file)
    }
}

#[derive(Debug)]
struct File {
    size: usize,
}

impl File {
    fn new(size: usize) -> File {
        File { size }
    }
}

fn main() {
    let root = Rc::new(RefCell::new(Directory::root()));
    let current = Rc::clone(&root);

    let child = Rc::new(RefCell::new(Directory::new(Rc::clone(&current))));
    root.borrow_mut().add_child(Rc::clone(&child));

    child.as_ref().borrow_mut().add_file(File::new(100));
    root.as_ref().borrow_mut().add_file(File::new(100));
    root.as_ref().borrow_mut().add_file(File::new(100));
    root.as_ref().borrow_mut().add_file(File::new(100));

    println!("Size: {}", root.as_ref().borrow().size())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
