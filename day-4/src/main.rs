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
            .into_iter()
            .map(|assignment| split_into_vec(assignment))
            .filter(|c| are_overlapping(&c.0, &c.1))
            .count();

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

fn split_into_vec(assignment_line: String) -> (Vec<usize>, Vec<usize>) {
    let splits = assignment_line.split_once(",").unwrap();
    let first = to_coverage(splits.0);
    let second = to_coverage(splits.1);
    println!("{:?} {:?}", first, second);
    return (first, second);
}

fn to_coverage(assignment: &str) -> Vec<usize> {
    let part = assignment.split_once("-").unwrap();
    return (part.0.parse::<usize>().unwrap()..part.1.parse::<usize>().unwrap() + 1).collect();
}

fn is_one_covering_other(first: &Vec<usize>, second: &Vec<usize>) -> bool {
    first.first().unwrap() <= second.first().unwrap()
        && first.last().unwrap() >= second.last().unwrap()
}

fn are_overlapping(first: &Vec<usize>, second: &Vec<usize>) -> bool {
    !(first.last().unwrap() < second.first().unwrap()
        || second.last().unwrap() < first.first().unwrap())
}
