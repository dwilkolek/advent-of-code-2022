#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let entries: Vec<String> = lines
            .map(|line| match line {
                Ok(line) => line,
                _ => panic!("Wtf"),
            })
            .collect();
        let result: usize = entries
            .chunks(3)
            .into_iter()
            .map(|group| find_common(group.clone().to_vec()))
            .map(|c| priority_from_char(c))
            .sum();

        println!("{:?}", result);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_common(mut list: Vec<String>) -> char {
    let mut common = vec![];
    let mut reference: Vec<char> = list.pop().unwrap().chars().collect();
    for entry in list {
        common = vec![];
        for a_char in entry.chars() {
            if reference.contains(&a_char) && !common.contains(&a_char) {
                common.push(a_char)
            }
        }
        reference = common.clone();
    }
    if common.len() > 1 {
        panic!("too many common characters")
    }
    common
        .get(0)
        .expect("Expected at least 1 common character")
        .to_owned()
}

fn priority_from_char(a: char) -> usize {
    //A-Z: 66-90 -> 27-52
    //a-z: 97-122 -> 1-26
    let ascii_dec = a as usize;
    return if ascii_dec < 94 {
        ascii_dec - 38
    } else {
        ascii_dec - 96
    };
}
